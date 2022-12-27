import asyncio
import json
import shutil
from amqp_api_client_py import amqp_output_api
import git_helper

GIT_LOCAL_REPOSITORY_PATH = "./latest_definition_repo"
TIMEOUT_AFTER_SECONDS = 5

async def main():
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

    await asyncio.wait_for(output_api.connect(), TIMEOUT_AFTER_SECONDS)
    serialized_response = await asyncio.wait_for(output_api.read(), TIMEOUT_AFTER_SECONDS)

    response = json.loads(serialized_response)

    latest_definition_version = git_helper.get_latest_definition_version(GIT_LOCAL_REPOSITORY_PATH)

    shutil.rmtree(GIT_LOCAL_REPOSITORY_PATH)

    if response["version"] != latest_definition_version:
        print(f"expected version '{latest_definition_version}' got '{response['version']}'")
        exit(1)


if __name__ == "__main__":
    asyncio.run(main())