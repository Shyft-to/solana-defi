import logging
import os
import signal
import threading

import grpc
from dotenv import load_dotenv

import geyser_pb2
import geyser_pb2_grpc

load_dotenv()

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s  %(levelname)-5s  %(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S",
)
log = logging.getLogger(__name__)

RECONNECT_DELAY_SECS = 5


def _create_secure_channel(endpoint: str, token: str | None, big_data: bool = False) -> grpc.Channel:
    auth = grpc.metadata_call_credentials(lambda _, callback: callback((("x-token", token),), None))
    ssl_creds = grpc.ssl_channel_credentials()
    combined_creds = grpc.composite_channel_credentials(ssl_creds, auth)
    options = [("grpc.max_receive_message_length", 2**25)] if big_data else []
    #return grpc.secure_channel(endpoint, credentials=combined_creds, options=options)
    return grpc.secure_channel(endpoint, credentials=combined_creds)


def _request_iter(stop: threading.Event):
    request = geyser_pb2.SubscribeRequest(
        commitment=geyser_pb2.CommitmentLevel.CONFIRMED,
    )
    request.blocks_meta["blocks_meta"].CopyFrom(geyser_pb2.SubscribeRequestFilterBlocksMeta())
    yield request
    stop.wait()


def _run_once(endpoint: str, token: str | None, stop: threading.Event) -> None:
    channel = _create_secure_channel(endpoint, token)
    try:
        stub = geyser_pb2_grpc.GeyserStub(channel)
        responses = stub.Subscribe(_request_iter(stop))
        log.info("stream connected (commitment=CONFIRMED)")
        for msg in responses:
            if stop.is_set():
                return
            if msg.HasField("block_meta"):
                log.info("slot=%d", msg.block_meta.slot)
        log.warning("stream closed by server")
    except grpc.RpcError as e:
        if not stop.is_set():
            log.error("stream disconnected — code=%s  detail=%s", e.code(), e.details())
    finally:
        channel.close()


def stream_blocks_meta(endpoint: str, token: str | None) -> None:
    stop = threading.Event()

    def _handle_signal(*_):
        log.info("shutting down")
        stop.set()

    signal.signal(signal.SIGINT, _handle_signal)
    signal.signal(signal.SIGTERM, _handle_signal)

    attempt = 0
    while not stop.is_set():
        attempt += 1
        log.info("connecting to %s (attempt %d)", endpoint, attempt)
        _run_once(endpoint, token, stop)

        if stop.is_set():
            break

        log.info("reconnecting in %ds...", RECONNECT_DELAY_SECS)
        stop.wait(timeout=RECONNECT_DELAY_SECS)


if __name__ == "__main__":
    endpoint = os.environ.get("GRPC_ENDPOINT")
    if not endpoint:
        raise SystemExit("GRPC_ENDPOINT is required")

    token = os.environ.get("GRPC_X_TOKEN")
    stream_blocks_meta(endpoint, token)
