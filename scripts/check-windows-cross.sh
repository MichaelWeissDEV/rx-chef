#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image=${RXCHEF_WINDOWS_IMAGE:-rxchef-windows-check}

docker build \
    -f "$repo_root/docker/Dockerfile.windows" \
    -t "$image" "$repo_root"

docker run --rm \
    -v "$repo_root:/src" \
    -v rxchef-windows-cargo-git:/root/.cargo/git \
    -v rxchef-windows-cargo-registry:/root/.cargo/registry \
    -v rxchef-windows-target:/tmp/rxchef-windows-target \
    "$image"
