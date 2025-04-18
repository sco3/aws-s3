#!/usr/bin/env -S uv run
""" tests with real s3 """
import asyncio

from aioboto3 import Session


async def list_folders_with_paginator(bucket: str, prefix: str = "") -> None:
    """test folders"""
    session = Session()
    async with session.client("s3") as s3:
        paginator = s3.get_paginator("list_objects_v2")
        pr = paginator.paginate(Bucket=bucket, Prefix=prefix, Delimiter="/")
        async for page in pr:
            if "CommonPrefixes" in page:
                for prefix in page["CommonPrefixes"]:
                    prefix_name = prefix["Prefix"]
                    print(f"{prefix_name}")


if __name__ == "__main__":
    asyncio.run(list_folders_with_paginator("dz-bucket-1234", "agent_"))
