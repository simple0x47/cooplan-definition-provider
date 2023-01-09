import json
import socket
import definition.get_definition_by_version
import asyncio
import os

TEST_TIMEOUT_AFTER_SECONDS_ENV = "TEST_TIMEOUT_AFTER_SECONDS"

async def main():
    print("[STATE_OUTPUT] Starting")
    config_file = open("./config.json")

    config = json.load(config_file)

    output_sender_path = config["state_tracking_config"]["state_output_sender_path"]
    output_receiver_path = config["state_tracking_config"]["state_output_receiver_path"]

    print("[STATE_OUTPUT] Creating socket")
    receiver_sock = socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM)
    receiver_sock.bind(output_receiver_path)

    buffer = bytes()

    timeout_after = int(os.environ.get(TEST_TIMEOUT_AFTER_SECONDS_ENV, 15))

    print("[STATE_OUTPUT] Waiting for get definition by version")
    await asyncio.wait_for(definition.get_definition_by_version.main(), timeout_after)

    print("[STATE_OUTPUT] Waiting for state output")
    data = receiver_sock.recv(1024)
    buffer += data

    print("[STATE_OUTPUT] Received state output")
    tracked_data = json.loads(buffer.decode("utf-8"))
    print(f"TrackedData: {tracked_data}")

    assert(tracked_data["id"] == "definition")
    assert(tracked_data["state"] == "Valid")


if __name__ == "__main__":
    asyncio.run(main())
