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
        print(f"Zieladresse empfangen: {target_address}")

        # Empfange Bilddatei
        with open('target_image.jpg', 'wb') as f:
            while True:
                bytes_read = client_socket.recv(4096)
                if not bytes_read:
                    break
                f.write(bytes_read)
        print("Bilddatei empfangen und gespeichert als 'target_image.jpg'")
    except Exception as e:
        print(f"Fehler beim Empfangen von Daten: {e}")
    finally:
        pass
        #client_socket.close()

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

        # Check if the process is still running and kill it
        if process.poll() is None:
            print(f"Process {process.pid} is still running. Killing it.")
            os.kill(process.pid, signal.SIGTERM)
            # Alternatively, use process.terminate() or process.kill() if SIGTERM is not appropriate
        else:
            print(f"Process {process.pid} has already terminated.")
        
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    while True:
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.connect((SERVER_HOST, SERVER_PORT))
        receive_data(client_socket)
        time.sleep(3)
        binary_path = "./result/bin/pixelpwnr"  # Replace with the actual binary path
        args = [target_address, "--image",  "/home/timl/SC/pixelpwnr/target_image.jpg"]  # Replace with actual arguments
        timeout = 10  # Time in seconds after which to kill the binary

        run_and_maybe_kill(binary_path, args, timeout)