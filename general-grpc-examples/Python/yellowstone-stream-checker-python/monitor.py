import asyncio
import logging
import time
from typing import Optional

from config import Config
from slack import send_idle_alert

log = logging.getLogger(__name__)


class LastSeen:
    """Tracks the monotonic timestamp of the last received gRPC message."""

    __slots__ = ("_ts", "count")

    def __init__(self) -> None:
        self._ts: float = 0.0
        self.count: int = 0

    def touch(self) -> None:
        self._ts = time.monotonic()
        self.count += 1

    def idle_secs(self) -> Optional[float]:
        if self._ts == 0.0:
            return None
        return time.monotonic() - self._ts


class StreamWatch:
    def __init__(self, name: str, last_seen: LastSeen) -> None:
        self.name = name
        self.last_seen = last_seen
        self._alerted = False


async def idle_monitor(cfg: Config, watches: list[StreamWatch]) -> None:
    while True:
        await asyncio.sleep(30)

        for w in watches:
            idle = w.last_seen.idle_secs()

            if idle is None:
                continue  # stream hasn't connected yet

            if idle >= cfg.idle_timeout_secs:
                if not w._alerted:
                    w._alerted = True
                    log.info("stream idle  name=%s  idle_secs=%.0f", w.name, idle)

                    if cfg.slack_webhook_url:
                        try:
                            await send_idle_alert(cfg.slack_webhook_url, w.name, int(idle))
                        except Exception as e:
                            log.warning("Slack alert failed: %s", e)
            else:
                # Activity resumed — clear flag so the next idle fires a fresh alert.
                w._alerted = False
