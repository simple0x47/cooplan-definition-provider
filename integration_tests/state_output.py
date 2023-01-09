import json
import socket


def main():
    config_file = open("./config.json")

    config = json.load(config_file)

    output_sender_path = config["state_tracking_config"]["state_output_sender_path"]
    output_receiver_path = config["state_tracking_config"]["state_output_receiver_path"]

    receiver_sock = socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM)
    receiver_sock.bind(output_receiver_path)

    buffer = bytes()

    # TODO: run get definition by version before receiving state

    data = receiver_sock.recv(1024)
    buffer += data

    tracked_data = json.loads(buffer.decode("utf-8"))

    assert(tracked_data["id"] == "definition")
    assert(tracked_data["state"] == "Valid")

if __name__ == "__main__":
    main()
