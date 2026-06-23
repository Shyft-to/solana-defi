import asyncio
import logging

import geyser_pb2
import geyser_pb2_grpc

from channel import make_channel
from config import Config
from monitor import LastSeen

log = logging.getLogger(__name__)

HEARTBEAT_INTERVAL = 30


async def _heartbeat(last_seen: LastSeen) -> None:
    while True:
        await asyncio.sleep(HEARTBEAT_INTERVAL)
        idle = last_seen.idle_secs()
        if idle is None:
            log.info("[transactions] connected — no messages received yet (total: 0)")
        else:
            log.info("[transactions] connected — last message %.0fs ago (total: %d)", idle, last_seen.count)


async def _run_once(cfg: Config, last_seen: LastSeen) -> None:
    log.info("[transactions] connecting to %s", cfg.grpc_endpoint)

    channel = make_channel(cfg)
    heartbeat_task = None
    try:
        stub = geyser_pb2_grpc.GeyserStub(channel)

        request = geyser_pb2.SubscribeRequest(
            commitment=geyser_pb2.CommitmentLevel.PROCESSED,
        )
        tx_filter = request.transactions["txn"]
        tx_filter.vote = False
        tx_filter.failed = False
        tx_filter.account_include.extend(cfg.account_include)

        call = stub.Subscribe()
        await call.write(request)

        log.info(
            "[transactions] connected (commitment=PROCESSED, accounts=%s)",
            cfg.account_include,
        )
        heartbeat_task = asyncio.create_task(_heartbeat(last_seen))

        async for _ in call:
            last_seen.touch()

        log.info("[transactions] stream closed by server")
    finally:
        if heartbeat_task:
            heartbeat_task.cancel()
        await channel.close()


async def spawn(cfg: Config, last_seen: LastSeen) -> None:
    attempt = 0
    while True:
        attempt += 1
        try:
            await _run_once(cfg, last_seen)
            log.info("[transactions] reconnecting (attempt %d)...", attempt + 1)
        except Exception as e:
            log.info("[transactions] disconnected — %s", e)
            log.info("[transactions] reconnecting in 3s (attempt %d)...", attempt + 1)
            await asyncio.sleep(3)
