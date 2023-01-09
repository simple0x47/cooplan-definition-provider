import json
import socket
import definition.get_definition_by_version
import asyncio

TIMEOUT_AFTER_SECONDS = 5

async def main():
    config_file = open("./config.json")

    config = json.load(config_file)

    output_sender_path = config["state_tracking_config"]["state_output_sender_path"]
    output_receiver_path = config["state_tracking_config"]["state_output_receiver_path"]

    receiver_sock = socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM)
    receiver_sock.bind(output_receiver_path)

    buffer = bytes()

    await asyncio.wait_for(definition.get_definition_by_version.main(), TIMEOUT_AFTER_SECONDS)

    data = receiver_sock.recv(1024)
    buffer += data

    tracked_data = json.loads(buffer.decode("utf-8"))

    assert(tracked_data["id"] == "definition")
    assert(tracked_data["state"] == "Valid")


if __name__ == "__main__":
    asyncio.run(main())
