#!/usr/bin/env python3
"""Hash locally produced packages; never upload or publish."""
import hashlib
import json
import os
import pathlib
import platform
import subprocess

target = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", "target"))
destination = pathlib.Path(os.environ.get("RXCHEF_DISTRIBUTION_DIR", str(target / "distributions")))
destination.mkdir(parents=True, exist_ok=True)
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"]))
version = next(p["version"] for p in metadata["packages"] if p["name"] == "rxchef")
commit = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
packages = list(destination.glob("*.tar.gz"))
for extension in ("dmg", "deb", "rpm", "AppImage", "exe", "msi"):
    packages.extend((target / "release/bundle").rglob("*." + extension))
records = []
for package in sorted(packages):
    digest = hashlib.sha256()
    with package.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    checksum = digest.hexdigest()
    package.with_suffix(package.suffix + ".sha256").write_text(f"{checksum}  {package.name}\n")
    records.append({"filename": package.name, "path": str(package.resolve()), "version": version,
                    "os": platform.system(), "architecture": platform.machine(), "commit": commit,
                    "bytes": package.stat().st_size, "sha256": checksum, "build": "PASS",
                    "installation": "NOT TESTED", "smoke": "NOT TESTED"})
manifest = destination / f"artifacts-{platform.system()}-{platform.machine()}.json"
manifest.write_text(json.dumps(records, indent=2) + "\n")
print(f"{len(records)} packages: {manifest}")
if not records:
    raise SystemExit("No release packages found")
