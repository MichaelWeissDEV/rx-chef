#!/usr/bin/env python3
"""Run a local check and retain its exit status and reproducible evidence."""
import argparse
import datetime
import json
import pathlib
import platform
import re
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--label", required=True)
parser.add_argument("--features", default="default")
parser.add_argument("--directory", default="target/rc-evidence")
parser.add_argument("command", nargs=argparse.REMAINDER)
args = parser.parse_args()
command = args.command[1:] if args.command[:1] == ["--"] else args.command
if not command:
    parser.error("a command is required")
directory = pathlib.Path(args.directory)
directory.mkdir(parents=True, exist_ok=True)
stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
name = stamp + "-" + re.sub(r"[^a-zA-Z0-9_-]", "-", args.label)
log = directory / (name + ".log")

def version(command):
    result = subprocess.run(command, capture_output=True, text=True)
    return result.stdout.strip() if result.returncode == 0 else "unavailable"

record = {
    "label": args.label, "command": command, "platform": platform.system(),
    "architecture": platform.machine(), "rust_version": version(["rustc", "--version"]),
    "node_version": version(["node", "--version"]),
    "commit": version(["git", "rev-parse", "HEAD"]),
    "git_status": version(["git", "status", "--short"]), "features": args.features,
    "start_time": stamp, "log_path": str(log.resolve()),
}
start = time.monotonic()
with log.open("w") as output:
    try:
        result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT)
        code = result.returncode
    except OSError as error:
        output.write(str(error) + "\n")
        code = 127
record.update(duration_seconds=round(time.monotonic() - start, 3), exit_code=code,
              result="PASS" if code == 0 else "FAIL")
content = re.sub(r"\x1b\[[0-9;]*m", "", log.read_text(errors="replace"))
counts = re.findall(r"test result: .*? (\d+) passed; (\d+) failed; (\d+) ignored", content)
vitest = re.findall(r"^\s*Tests\s+(\d+) passed", content, re.MULTILINE)
playwright = re.findall(r"^\s*(\d+) passed \(", content, re.MULTILINE)
python = re.findall(r"Ran (\d+) tests? in .*\n\nOK", content)
reported = bool(counts or vitest or playwright or python)
record.update(passed_tests=(sum(int(c[0]) for c in counts) + sum(map(int, vitest + playwright + python))) if reported else None,
              failed_tests=sum(int(c[1]) for c in counts) if counts else None,
              skipped_tests=sum(int(c[2]) for c in counts) if counts else None)
(directory / (name + ".json")).write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record, indent=2), flush=True)
raise SystemExit(code if code >= 0 else 128 - code)
