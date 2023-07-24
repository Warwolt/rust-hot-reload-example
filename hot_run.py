import signal
import subprocess
import sys

def signal_handler(sig, frame):
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

processes = [
    subprocess.Popen(["cargo", "watch", "-w", "lib", "-x", "build -p lib"]),
    subprocess.Popen(["cargo", "run"]),
]

for p in processes:
    p.wait()
