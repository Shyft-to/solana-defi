#!/usr/bin/env bash
# Generates Python stubs from local proto files.
# Run once before `python main.py`.
set -euo pipefail

echo "Generating Python stubs..."
python -m grpc_tools.protoc \
    -I . \
    --python_out=. \
    --grpc_python_out=. \
    solana-storage.proto \
    geyser.proto

echo "Done — geyser_pb2.py and geyser_pb2_grpc.py written to project root."
