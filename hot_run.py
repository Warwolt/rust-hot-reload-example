import signal
import subprocess
import sys

def signal_handler(sig, frame):
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

processes = [
    subprocess.Popen(["cargo", "watch", "-w", "app", "-x", "build -p app"]),
    subprocess.Popen(["cargo", "run"]),
]

while True:
    for p in processes:
        code = p.poll()
        if code != None:
            for p in processes:
                p.terminate()
            sys.exit(code)
