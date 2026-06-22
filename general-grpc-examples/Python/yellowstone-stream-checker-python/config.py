import os
from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Config:
    grpc_endpoint: str
    grpc_x_token: Optional[str]
    account_include: list[str]
    idle_timeout_secs: int
    slack_webhook_url: Optional[str]

    @classmethod
    def from_env(cls) -> "Config":
        endpoint = os.environ.get("GRPC_ENDPOINT")
        if not endpoint:
            raise ValueError("GRPC_ENDPOINT is required")

        token = os.environ.get("GRPC_X_TOKEN")

        accounts_raw = os.environ.get("ACCOUNT_INCLUDE", "")
        accounts = [a.strip() for a in accounts_raw.split(",") if a.strip()]

        idle = int(os.environ.get("IDLE_TIMEOUT_SECS", "120"))

        slack = os.environ.get("SLACK_WEBHOOK_URL")

        return cls(
            grpc_endpoint=endpoint,
            grpc_x_token=token,
            account_include=accounts,
            idle_timeout_secs=idle,
            slack_webhook_url=slack,
        )
