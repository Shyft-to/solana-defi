import asyncio
import logging
import signal

from dotenv import load_dotenv

import blocks_meta
import transactions
from config import Config
from monitor import LastSeen, StreamWatch, idle_monitor

load_dotenv()

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(levelname)-5s  %(name)s  %(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S",
)
log = logging.getLogger(__name__)


async def main() -> None:
    cfg = Config.from_env()

    log.info(
        "silent-stream-detector starting  idle_timeout=%ds  slack=%s",
        cfg.idle_timeout_secs,
        "yes" if cfg.slack_webhook_url else "no",
    )

    bm_last_seen = LastSeen()
    tx_last_seen = LastSeen()

    watches = [
        StreamWatch("blocks_meta", bm_last_seen),
        StreamWatch("transactions", tx_last_seen),
    ]

    stop = asyncio.Event()
    loop = asyncio.get_running_loop()
    loop.add_signal_handler(signal.SIGINT, stop.set)
    loop.add_signal_handler(signal.SIGTERM, stop.set)

    tasks = [
        asyncio.create_task(blocks_meta.spawn(cfg, bm_last_seen), name="blocks_meta"),
        asyncio.create_task(transactions.spawn(cfg, tx_last_seen), name="transactions"),
        asyncio.create_task(idle_monitor(cfg, watches), name="monitor"),
    ]

    await stop.wait()
    log.info("shutting down")

    for t in tasks:
        t.cancel()
    await asyncio.gather(*tasks, return_exceptions=True)


if __name__ == "__main__":
    asyncio.run(main())
