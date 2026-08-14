#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image=${RXCHEF_RELEASE_IMAGE:-rxchef-release-check}

docker build --platform linux/amd64 \
    -f "$repo_root/docker/Dockerfile.release" \
    -t "$image" "$repo_root"

docker run --rm --platform linux/amd64 \
    -v "$repo_root:/src" \
    -v rxchef-release-cargo-git:/root/.cargo/git \
    -v rxchef-release-cargo-registry:/root/.cargo/registry \
    -v rxchef-release-target:/tmp/rxchef-target \
    "$image" "$@"
