import asyncio
import json
from amqp_api_client_py import amqp_input_api
from cooplan_integration_test_boilerplate import test
import os

GET_DEFINITION_VERSION = "6bc4697553f0511780480ddae602a636802d3cdc"
TEST_TIMEOUT_AFTER_SECONDS_ENV = "TEST_TIMEOUT_AFTER_SECONDS"

REQUEST = {
    "header": {
        "element": "definition",
        "action": "get"
    },
    "version": GET_DEFINITION_VERSION,
}

REQUEST_AMQP_CONFIG = {
    "queue": {
        "name": "definition",
        "passive": False,
        "durable": False,
        "exclusive": False,
        "auto_delete": True,
        "nowait": False,
        "arguments": {}
    },
    "channel": {
        "publish": {
            "mandatory": False,
            "immediate": False,
            "timeout": None
        }
    }
}

RESPONSE_AMQP_CONFIG = {
    "queue": {
        "name": "",
        "passive": False,
        "durable": False,
        "exclusive": False,
        "auto_delete": True,
        "nowait": False,
        "arguments": {}
    },
    "channel": {
        "consume": {
            "no_ack": False,
            "exclusive": False,
            "arguments": {},
            "consumer_tag": None,
            "timeout": None
        }
    }
}


async def main():
    print("[GET_DEFINITION_BY_VERSION] Starting")
    test.init_request(REQUEST)

    print("[GET_DEFINITION_BY_VERSION] Creating AMQP input API")
    input_api = amqp_input_api.AmqpInputApi(REQUEST_AMQP_CONFIG, RESPONSE_AMQP_CONFIG)

    timeout_after = int(os.environ.get(TEST_TIMEOUT_AFTER_SECONDS_ENV, 15))
    print("[GET_DEFINITION_BY_VERSION] Connecting to AMQP")
    await asyncio.wait_for(input_api.connect(), timeout_after)

    print("[GET_DEFINITION_BY_VERSION] Sending request")
    serialized_definition = await asyncio.wait_for(input_api.send_request(REQUEST), timeout_after)

    print("[GET_DEFINITION_BY_VERSION] Received response")
    definition = json.loads(serialized_definition)

    if definition["Ok"]["version"] != GET_DEFINITION_VERSION:
        print(f"expected version '{GET_DEFINITION_VERSION}' got '{definition['version']}'")
        exit(1)

    exit(0)

if __name__ == "__main__":
    asyncio.run(main())
