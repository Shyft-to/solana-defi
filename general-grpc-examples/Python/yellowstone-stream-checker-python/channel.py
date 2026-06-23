import grpc
import grpc.aio

from config import Config


def make_channel(cfg: Config) -> grpc.aio.Channel:
    endpoint = cfg.grpc_endpoint
    for prefix in ("https://", "http://"):
        if endpoint.startswith(prefix):
            endpoint = endpoint[len(prefix):]
            break

    ssl_creds = grpc.ssl_channel_credentials()

    if cfg.grpc_x_token:
        def _add_x_token(context, callback):
            callback([("x-token", cfg.grpc_x_token)], None)
        call_creds = grpc.metadata_call_credentials(_add_x_token)
        creds = grpc.composite_channel_credentials(ssl_creds, call_creds)
    else:
        creds = ssl_creds

    return grpc.aio.secure_channel(
        endpoint,
        creds,
        options=[
            ("grpc.max_receive_message_length", 128 * 1024 * 1024),
            ("grpc.keepalive_time_ms", 30_000),
            ("grpc.keepalive_timeout_ms", 10_000),
            ("grpc.keepalive_permit_without_calls", 1),
        ],
    )
