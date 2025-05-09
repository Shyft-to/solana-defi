import sys
from pathlib import Path

current_dir = str(Path(__file__).parent)
if current_dir not in sys.path:
        sys.path.append(current_dir)

from . import geyser_pb2
from . import geyser_pb2_grpc
from . import solana_storage_pb2
from . import solana_storage_pb2_grpc