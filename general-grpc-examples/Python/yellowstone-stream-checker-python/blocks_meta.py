import asyncio
import logging
import time

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
            log.info("[blocks_meta] connected — no messages received yet (total: 0)")
        else:
            log.info("[blocks_meta] connected — last message %.0fs ago (total: %d)", idle, last_seen.count)


async def _run_once(cfg: Config, last_seen: LastSeen) -> None:
    log.info("[blocks_meta] connecting to %s", cfg.grpc_endpoint)

    channel = make_channel(cfg)
    heartbeat_task = None
    try:
        stub = geyser_pb2_grpc.GeyserStub(channel)

        request = geyser_pb2.SubscribeRequest(
            commitment=geyser_pb2.CommitmentLevel.CONFIRMED,
        )
        request.blocks_meta["blocks_meta"].CopyFrom(
            geyser_pb2.SubscribeRequestFilterBlocksMeta()
        )

        call = stub.Subscribe()
        await call.write(request)

        log.info("[blocks_meta] connected (commitment=CONFIRMED)")
        heartbeat_task = asyncio.create_task(_heartbeat(last_seen))

        async for _ in call:
            last_seen.touch()

        log.info("[blocks_meta] stream closed by server")
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
            log.info("[blocks_meta] reconnecting (attempt %d)...", attempt + 1)
        except Exception as e:
            log.info("[blocks_meta] disconnected — %s", e)
            log.info("[blocks_meta] reconnecting in 3s (attempt %d)...", attempt + 1)
            await asyncio.sleep(3)
