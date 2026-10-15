#!/usr/bin/env python3
"""Hash locally produced packages; never upload or publish."""
import hashlib
import argparse
import json
import os
import pathlib
import platform
import subprocess
import shutil
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--stage", type=pathlib.Path, help="Prepare a fresh flat directory for the release host's artifacts/* glob")
args = parser.parse_args()

target = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", "target"))
destination = pathlib.Path(os.environ.get("RXCHEF_DISTRIBUTION_DIR", str(target / "distributions")))
destination.mkdir(parents=True, exist_ok=True)
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"]))
version = next(p["version"] for p in metadata["packages"] if p["name"] == "rxchef")
commit = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
packages = list(destination.glob(f"rxchef-{version}-*.tar.gz"))
for extension in ("dmg", "deb", "rpm", "AppImage", "exe", "msi"):
    for package in (target / "release/bundle").rglob("*." + extension):
        # Local build directories may retain installers from earlier versions.
        # Never relabel or upload those as the current release.
        current = bool(re.search(r"-" + re.escape(version) + r"-\d+\.", package.name)) if extension == "rpm" else f"_{version}_" in package.name
        if current:
            packages.append(package)
records = []
for package in sorted(packages):
    digest = hashlib.sha256()
    with package.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    checksum = digest.hexdigest()
    package.with_suffix(package.suffix + ".sha256").write_text(f"{checksum}  {package.name}\n")
    records.append({"filename": package.name, "path": str(package.resolve()), "version": version,
                    "os": platform.system(), "architecture": platform.machine(), "inventory_commit": commit,
                    "build_commit": None,
                    "bytes": package.stat().st_size, "sha256": checksum, "build": "PASS",
                    "installation": "NOT TESTED", "smoke": "NOT TESTED"})
manifest = destination / f"artifacts-{platform.system()}-{platform.machine()}.json"
manifest.write_text(json.dumps(records, indent=2) + "\n")
print(f"{len(records)} packages: {manifest}")
if not records:
    raise SystemExit("No release packages found")
if args.stage:
    # upload-artifact otherwise preserves differing parent directories, while
    # cargo-dist's generated GitHub release step consumes a flat artifacts/*.
    files = [manifest]
    for package in packages:
        files.extend([package, package.with_suffix(package.suffix + ".sha256")])
    if len({file.name for file in files}) != len(files):
        raise SystemExit("Duplicate artifact basename; refusing to flatten")
    args.stage.mkdir(parents=True, exist_ok=False)
    for file in files:
        shutil.copy2(file, args.stage / file.name)
    print(f"Staged {len(files)} files in {args.stage}")
