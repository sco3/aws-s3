#!/usr/bin/env -S uv run
""" tests with real s3 """

import boto3
from mypy_boto3_s3.client import S3Client
from mypy_boto3_s3.paginator import ListObjectsV2Paginator


def list_folders_with_paginator(bucket: str, prefix: str = "") -> None:
    """test folders"""
    s3: S3Client = boto3.client("s3")

    paginator: ListObjectsV2Paginator = s3.get_paginator("list_objects_v2")

    operation_params = {"Bucket": bucket, "Prefix": prefix, "Delimiter": "/"}

    for page in paginator.paginate(**operation_params):  # pyre-ignore[6]
        if "CommonPrefixes" in page:
            for prefix in page["CommonPrefixes"]:
                prefix_name = prefix["Prefix"]
                print(f"{prefix_name}")


if __name__ == "__main__":
    list_folders_with_paginator("dz-bucket-1234", "agent_")
