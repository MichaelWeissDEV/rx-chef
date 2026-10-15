#!/usr/bin/env python3
import base64
import json
import subprocess
import sys

binary = sys.argv[1]
all_bytes = bytes(range(256)) + b"\x00\xff\xc0"
requests = [
    {"jsonrpc": "2.0", "id": 1, "method": "ping"},
    {"jsonrpc": "2.0", "id": 2, "method": "operations"},
    {"jsonrpc": "2.0", "id": 3, "method": "describe", "params": {"operation": "From Base64"}},
    {"jsonrpc": "2.0", "id": 4, "method": "run", "params": {"operation": "From Base64", "input": "SGVsbG8="}},
    {"jsonrpc": "2.0", "id": 5, "method": "bake", "params": {"input": "Hello", "recipe": [{"op": "To Upper case"}, {"op": "To Base64"}]}},
    {"jsonrpc": "2.0", "id": 6, "method": "not-a-method"},
    {"jsonrpc": "2.0", "id": 7, "method": "run", "params": {"operation": "From Base64", "input": base64.b64encode(all_bytes).decode()}},
    {"jsonrpc": "2.0", "id": 8, "method": "bake", "params": {"input_base64": base64.b64encode(all_bytes).decode(), "recipe": []}},
    {"jsonrpc": "2.0", "method": "ping"},
    {"jsonrpc": "2.0", "id": 9, "method": "ping"},
    {"jsonrpc": "2.0", "id": 10, "method": "shutdown"},
    {"jsonrpc": "2.0", "id": 11, "method": "ping"},
]
payload = "{malformed-json}\n" + "".join(json.dumps(request) + "\n" for request in requests)
process = subprocess.run(
    [binary, "serve", "--stdio"], input=payload.encode(), capture_output=True, check=True, timeout=30
)
if process.stderr:
    raise SystemExit(f"server polluted stderr: {process.stderr!r}")
responses = [json.loads(line) for line in process.stdout.splitlines()]
assert len(responses) == 11
assert responses.pop(0)["error"]["code"] == -32700
assert [response["id"] for response in responses] == list(range(1, 11))
assert responses[0]["result"]["protocol_version"] == 2
assert len(responses[1]["result"]) == 478
assert len({operation["id"] for operation in responses[1]["result"]}) == 478
assert responses[2]["result"]["name"] == "From Base64"
assert base64.b64decode(responses[3]["result"]["output_base64"]) == b"Hello"
assert base64.b64decode(responses[4]["result"]["output_base64"]) == b"SEVMTE8="
assert responses[5]["error"]["code"] == -32601
assert base64.b64decode(responses[6]["result"]["output_base64"]) == all_bytes
assert base64.b64decode(responses[7]["result"]["output_base64"]) == all_bytes
assert responses[8]["result"]["protocol_version"] == 2
assert responses[9]["result"]["shutdown"] is True
print("PASS JSONL protocol 2: malformed-input recovery, notifications, ordered responses, catalog, exact binary run/bake and shutdown")
