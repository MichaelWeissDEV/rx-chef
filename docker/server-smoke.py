#!/usr/bin/env python3
import base64
import json
import subprocess
import sys

binary = sys.argv[1]
requests = [
    {"jsonrpc": "2.0", "id": 1, "method": "ping"},
    {"jsonrpc": "2.0", "id": 2, "method": "operations"},
    {"jsonrpc": "2.0", "id": 3, "method": "describe", "params": {"operation": "From Base64"}},
    {"jsonrpc": "2.0", "id": 4, "method": "run", "params": {"operation": "From Base64", "input": "SGVsbG8="}},
    {"jsonrpc": "2.0", "id": 5, "method": "bake", "params": {"input": "Hello", "recipe": [{"op": "To Upper case"}, {"op": "To Base64"}]}},
    {"jsonrpc": "2.0", "id": 6, "method": "shutdown"},
]
payload = "".join(json.dumps(request) + "\n" for request in requests)
process = subprocess.run(
    [binary, "serve", "--stdio"], input=payload.encode(), capture_output=True, check=True
)
if process.stderr:
    raise SystemExit(f"server polluted stderr: {process.stderr!r}")
responses = [json.loads(line) for line in process.stdout.splitlines()]
assert len(responses) == 6
assert responses[0]["result"]["protocol_version"] == 1
assert len(responses[1]["result"]) == 478
assert responses[2]["result"]["name"] == "From Base64"
assert base64.b64decode(responses[3]["result"]["output_base64"]) == b"Hello"
assert base64.b64decode(responses[4]["result"]["output_base64"]) == b"SEVMTE8="
assert responses[5]["result"]["shutdown"] is True
