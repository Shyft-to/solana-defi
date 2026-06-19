# Subscribe to raydium pool updates using gRPC, in Python

## Installation

Clone this repository:

Navigate to this example directory:

```bash
cd solana-defi/Raydium/Python
```

Create a virtual environment:

```bash
python3 -m venv venv
```

Activate the virtual environment:

```bash
source venv/bin/activate # macOS / Linux
```
or 

```bash
venv\Scripts\activate.bat # Windows
```

Install the dependencies:

```bash
(venv) $ python3 -m pip install -r requirements.txt
```

## Usage

Set the gRPC url and token

```bash
    monitor = RaydiumMonitor(
        "your_grpc_url",
        "your_grpc_token"
    )
```

Run the script:

```bash
(venv) $ python3 main.py
```

## Advanced

If you prefer, you can generate all the stubs and data types using the following command:

```bash
(venv) $ python3 -python3 -m pip install grpcio-tools

(venv) $ python3 -m grpc_tools.protoc \
  -I./proto/ \
  --python_out=./generated \
  --pyi_out=./generated \
  --grpc_python_out=./generated \
  ./proto/*
```

This will generate the following files in your `generated` directory (which is where `main.py` currently imports from):
- geyser_pb2.py
- geyser_pb2.pyi
- geyser_pb2_grpc.py
- solana_storage_pb2.py
- solana_storage_pb2.pyi
- solana_storage_pb2_grpc.py