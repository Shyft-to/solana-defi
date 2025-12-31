import asyncio
import base58
import grpc
import logging
from typing import Iterator

import generated.geyser_pb2 as geyser_pb2
import generated.geyser_pb2_grpc as geyser_pb2_grpc

logger = logging.getLogger(__name__)

def SignatureTransactionFromJson(data):
    try:
        signature = base58.b58encode(bytes(data.signature)).decode()
        transaction = data.transaction
        signatures = base58.b58encode(bytes(transaction.signatures[0]))
        meta = data.meta
        return {'signature': signature, 'transaction': transaction, 'signatures': signatures, 'meta': meta}
    except Exception as e:
        return {'error': str(e)}

RAYDIUM_V4_LIQUIDITY_POOL_ADDRESS = '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'

class RaydiumMonitor:

    def __init__(self, endpoint: str, token: str) -> None:
        """
        Initializer. For information on initialization, refer to README.md.

        Args:
            endpoint: Shyft gRPC endpoint URL
            token: Shyft gRPC token address
        """
        self.endpoint = endpoint.replace('http://', '').replace('https://', '')
        self.token = token
        self.channel = self._create_secure_channel()
        self.stub = geyser_pb2_grpc.GeyserStub(self.channel)

    def _create_secure_channel(self) -> grpc.Channel:
        """Create a secure gRPC channel with authentication credentials."""
        auth = grpc.metadata_call_credentials(
            lambda context, callback: callback((("x-token", self.token),), None)
        )
        ssl_creds = grpc.ssl_channel_credentials()
        combined_creds = grpc.composite_channel_credentials(ssl_creds, auth)
        return grpc.secure_channel(self.endpoint, credentials=combined_creds)

    def request_iterator(self) -> Iterator[geyser_pb2.SubscribeRequest]:
        """
        Generate subscription requests for monitoring.

        Yields:
            geyser_pb2.SubscribeRequest: Configured subscription request
        """
        request = geyser_pb2.SubscribeRequest()
        request.transactions["all"].failed = False
        request.transactions["all"].vote = False
        request.transactions["all"].account_include.extend([RAYDIUM_V4_LIQUIDITY_POOL_ADDRESS])
        """
        You can add any other filters you need here, for example: 
            request.blocks...
            request.accounts...
            request.slots...
            ...
        """
        yield request

    def handle_update(self, update: geyser_pb2.SubscribeUpdate) -> None:
        """
        Process transaction updates from the subscription.
        This is where you process your response from the event stream.

        Args:
            update: Update message from the gRPC subscription
        """
        if hasattr(update, 'transaction'):
            print(SignatureTransactionFromJson(update.transaction.transaction))

    async def start_monitoring(self) -> None:
        """
        Start monitoring for streaming updates from the gRPC endpoint.
        Raises:
            grpc.RpcError: If gRPC communication fails
        """
        try:
            responses = self.stub.Subscribe(self.request_iterator())
            for response in responses:
                self.handle_update(response)
        except grpc.RpcError as e:
            logger.error(f"gRPC error occurred: {e}")
            raise
        finally:
            self.channel.close()

def main():
    logging.basicConfig(level=logging.INFO)
    monitor = RaydiumMonitor(
        "your_grpc_url",
        "your_grpc_token"
    )
    try:
        asyncio.run(monitor.start_monitoring())
    except KeyboardInterrupt:
        print("\nShutting down...")

if __name__ == "__main__":
    main()

