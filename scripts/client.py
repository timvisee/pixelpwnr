#!/usr/bin/env python

import socket
import time
import subprocess
import signal
import os

# Server-Einstellungen
SERVER_HOST = '127.0.0.1'
SERVER_PORT = 9999

target_address = None

def receive_data(client_socket):
    global target_address
    try:
        # Empfange Zieladresse
        target_address = client_socket.recv(1024).decode()

        # Empfange Bilddatei
        with open('target_image.jpg', 'wb') as f:
            while True:
                bytes_read = client_socket.recv(4096)
                if not bytes_read:
                    break
                f.write(bytes_read)
    except Exception as e:
        print(f"Fehler beim Empfangen von Daten: {e}")
    finally:
        client_socket.close()

def run_and_maybe_kill(binary_path, args, timeout):
    """
    Start a binary and maybe kill it after timeout seconds.

    :param binary_path: Path to the binary to be executed.
    :param args: List of arguments to pass to the binary.
    :param timeout: Time in seconds to wait before killing the binary.
    """
    try:
        # Start the binary
        process = subprocess.Popen([binary_path] + args)

        # Wait for the specified timeout
        time.sleep(timeout)

        # kill it
        os.kill(process.pid, signal.SIGTERM)
        
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    print("--- starting pixel client ---")
    print("-----------------------------")
    print("trying test connect to " + SERVER_HOST + ":" + str(SERVER_PORT) + " ...")
    
    try:
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.connect((SERVER_HOST, SERVER_PORT))
        client_socket.close()
    except Exception as e:
        print(e)
        print("connection has failed")
        exit(1)
    else:
        print("connection succesfully \n")
    
    while True:
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.connect((SERVER_HOST, SERVER_PORT))
        receive_data(client_socket)
        
        binary_path = "./result/bin/pixelpwnr"  # Replace with the actual binary path
        args = [target_address, "--image",  "/home/timl/SC/pixelpwnr/target_image.jpg"]  # Replace with actual arguments
        
        timeout = 30  # Time in seconds after which to kill the binary

        print("targeting: " + target_address)
        run_and_maybe_kill(binary_path, args, timeout)
