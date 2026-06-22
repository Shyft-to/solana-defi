import aiohttp


async def send_idle_alert(webhook_url: str, stream_name: str, idle_secs: int) -> None:
    minutes, secs = divmod(idle_secs, 60)
    text = f":warning: *{stream_name}* stream has been idle for {minutes}m {secs}s"

    async with aiohttp.ClientSession() as session:
        async with session.post(webhook_url, json={"text": text}) as resp:
            resp.raise_for_status()
