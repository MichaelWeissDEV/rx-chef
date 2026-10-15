#!/usr/bin/env python3
"""Install local tar archives in temporary roots and test real binaries twice."""
import argparse
import base64
import hashlib
import json
import os
import pathlib
import shutil
import socket
import subprocess
import tarfile
import tempfile
import time
import urllib.error
import urllib.request

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--directory", default="target/distributions")
parser.add_argument("--version", default="0.0.1")
args = parser.parse_args()
root = pathlib.Path(__file__).resolve().parent.parent
archives = list(pathlib.Path(args.directory).resolve().glob(f"rxchef-{args.version}-*.tar.gz"))
assert archives, "No archives to test"
graph = json.loads((root / "examples/graphs/three-inputs.rxgraph.json").read_text())
payload = bytes(range(256)) + b"\x00\xff\xc0"
bindings = {"payload": payload, "second": b"\xff\x00", "third": b"\xc0"}

def run(binary, arguments, cwd, data=None):
    result = subprocess.run([str(binary), *arguments], input=data, capture_output=True,
                            cwd=cwd, env={**os.environ, "RXCHEF_HOME": str(cwd / "store")}, timeout=30)
    assert result.returncode == 0, result.stderr.decode(errors="replace")
    return result.stdout

def request(url, path, value=None, token=True):
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = "Bearer local-archive-smoke-0123456789"
    req = urllib.request.Request(url + path, data=None if value is None else json.dumps(value).encode(), headers=headers)
    return urllib.request.urlopen(req, timeout=10)

results = []
for archive in archives:
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    assert archive.with_suffix(archive.suffix + ".sha256").read_text().split()[0] == digest
    with tempfile.TemporaryDirectory(prefix="rxchef-install-") as temporary:
        temp = pathlib.Path(temporary)
        install = temp / "installed"
        for attempt in range(2):
            install.mkdir()
            with tarfile.open(archive) as source:
                for member in source.getmembers():
                    if member.isdir():
                        assert member.name in (".", "./")
                        continue
                    assert member.isfile() and pathlib.PurePosixPath(member.name).name == member.name.removeprefix("./"), member.name
                    destination = install / pathlib.PurePosixPath(member.name).name
                    with source.extractfile(member) as stream:
                        destination.write_bytes(stream.read())
                    destination.chmod(member.mode)
            assert (install / "LICENSE").is_file()
            for binary in install.iterdir():
                name = binary.stem if binary.suffix == ".exe" else binary.name
                if name == "rxchef":
                    assert args.version.encode() in run(binary, ["--version"], temp)
                    assert run(binary, ["run", "From Base64"], temp, base64.b64encode(payload)) == payload
                    graph_file = temp / "graph.json"; graph_file.write_text(json.dumps(graph))
                    command = ["graph", "run", "--graph", str(graph_file)]
                    for key, data in bindings.items():
                        path = temp / (key + ".bin"); path.write_bytes(data)
                        command += ["--input", f"{key}={path}"]
                    for output in ["combined", "original", "encoded"]:
                        command += ["--output", f"{output}={temp / (output + '.bin')}"]
                    run(binary, command, temp)
                    assert (temp / "combined.bin").read_bytes() == b"".join(bindings.values())
                    assert (temp / "original.bin").read_bytes() == payload
                    assert (temp / "encoded.bin").read_bytes() == base64.b64encode(payload)
                elif name == "rxchef_tui":
                    assert b"graph" in run(binary, ["--help"], temp).lower()
                elif name == "rxchef-server":
                    assert args.version.encode() in run(binary, ["--version"], temp)
                    with socket.socket() as port:
                        port.bind(("127.0.0.1", 0)); address = port.getsockname()[1]
                    url = f"http://127.0.0.1:{address}"
                    process = subprocess.Popen([str(binary), "--listen", f"127.0.0.1:{address}"], cwd=temp,
                                               env={**os.environ, "RXCHEF_TOKEN": "local-archive-smoke-0123456789"},
                                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                    try:
                        for _ in range(100):
                            try:
                                with request(url, "/api/v1/health") as response:
                                    assert response.status == 200
                                break
                            except (urllib.error.URLError, ConnectionError):
                                assert process.poll() is None, "server exited"
                                time.sleep(0.05)
                        else:
                            raise AssertionError("server did not become ready")
                        with request(url, "/api/v1/capabilities") as response:
                            assert json.load(response)["protocol_version"] == 2
                        body = {"graph": graph, "inputs_base64": {key: base64.b64encode(data).decode() for key, data in bindings.items()}, "outputs": []}
                        with request(url, "/api/v1/graphs/execute", body) as response:
                            result = json.load(response)
                        outputs = {item["name"]: base64.b64decode(item["output_base64"]) for item in result["outputs"]}
                        assert outputs["combined"] == b"".join(bindings.values())
                        if "-web-" in archive.name:
                            with request(url, "/", token=False) as response:
                                assert b"<html" in response.read().lower()
                        else:
                            try:
                                request(url, "/")
                            except urllib.error.HTTPError as error:
                                assert error.code == 404
                            else:
                                raise AssertionError("headless archive unexpectedly serves web assets")
                    finally:
                        process.terminate()
                        process.wait(timeout=15)
            shutil.rmtree(install)
        results.append({"archive": archive.name, "sha256": digest, "install": "PASS", "uninstall": "PASS",
                        "reinstall": "PASS", "smoke": "PASS", "scope": "temporary archive installation on current host; TUI interactive PTY is a separate gate"})
        print("PASS", archive.name, "install, functional smoke, uninstall, reinstall", flush=True)
pathlib.Path(args.directory, "installation-smoke.json").write_text(json.dumps(results, indent=2) + "\n")
