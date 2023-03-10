import asyncio
import json
import shutil
from amqp_api_client_py import amqp_output_api
import git_helper
import os

GIT_LOCAL_REPOSITORY_PATH = "./latest_definition_repo"
TEST_TIMEOUT_AFTER_SECONDS_ENV = "TEST_TIMEOUT_AFTER_SECONDS"

async def main():
    print("[GET_LATEST_DEFINITION] Starting")
    output_api = amqp_output_api.AmqpOutputApi({
        "queue": {
            "name": "latest_definition",
            "passive": False,
            "durable": True,
            "exclusive": False,
            "auto_delete": False,
            "nowait": False,
            "arguments": {
                "x-queue-type": "stream"
            }
        },
        "channel": {
            "qos": {
                "prefetch_count": 1,
                "prefetch_size": None,
                "global": False,
                "timeout": None,
                "all_channels": None
            },
            "consume": {
                "no_ack": False,
                "exclusive": False,
                "arguments": {
                    "x-stream-offset": "last"
                },
                "consumer_tag": None,
                "timeout": None
            }
        }
    })

    timeout_after = int(os.environ.get(TEST_TIMEOUT_AFTER_SECONDS_ENV, 15))

    print("[GET_LATEST_DEFINITION] Waiting for latest definition")
    await asyncio.wait_for(output_api.connect(), timeout_after)
    serialized_response = await asyncio.wait_for(output_api.read(), timeout_after)

    print("[GET_LATEST_DEFINITION] Received latest definition")
    response = json.loads(serialized_response)

    print("[GET_LATEST_DEFINITION] Cloning the definitions repository")
    latest_definition_version = git_helper.get_latest_definition_version(GIT_LOCAL_REPOSITORY_PATH)

    shutil.rmtree(GIT_LOCAL_REPOSITORY_PATH)

    print("[GET_LATEST_DEFINITION] Completed cloning the definitions repository")
    if response["version"] != latest_definition_version:
        print(f"expected version '{latest_definition_version}' got '{response['version']}'")
        exit(1)

    exit(0)

if __name__ == "__main__":
    asyncio.run(main())