# stream-detector-sync

Connects to a Yellowstone gRPC node using synchronous `grpc` and streams `blocks_meta`, printing the slot number of each confirmed block.

---

## Requirements

- Python 3.10+
- A Yellowstone-compatible gRPC endpoint (host:port)
- An `x-token` auth token (if your node requires one)

---

## Setup

### 1. Create and activate a virtual environment

```bash
python3 -m venv .venv
source .venv/bin/activate
```

### 2. Install dependencies

```bash
pip install -r requirements.txt
```

### 3. Configure environment variables

Copy the example file and fill in your values:

```bash
cp .env.example .env
```

Edit `.env`:

```
GRPC_ENDPOINT=your-node.example.com:443
GRPC_X_TOKEN=your_token_here
```

- `GRPC_ENDPOINT` — **required.** The gRPC node address as `host:port`, no `https://` prefix.
- `GRPC_X_TOKEN` — **optional.** Auth token sent as `x-token` metadata. Leave it out or empty if your node does not require authentication.

---

## Run

```bash
python main.py
```

Expected output:

```
2026-06-23T00:00:00  INFO   connecting to your-node.example.com:443
2026-06-23T00:00:00  INFO   subscribed (commitment=CONFIRMED)
2026-06-23T00:00:00  INFO   slot=123456789
2026-06-23T00:00:00  INFO   slot=123456790
...
```

Stop with `Ctrl+C`.

---

## Project structure

```
.
├── main.py              # entry point
├── geyser_pb2.py        # generated protobuf types
├── geyser_pb2_grpc.py   # generated gRPC stub
├── requirements.txt
├── .env.example
└── .gitignore
```
