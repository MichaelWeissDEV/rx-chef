#!/usr/bin/env bash
#
# rx-chef release check.
#
# The authoritative gate is the Linux x86_64 container, which runs the complete
# set including FFI linking and packaging. Pass --host to run the portable
# subset directly on this machine when Docker is unavailable; skipped gates are
# reported explicitly in that mode.
#
#   scripts/release-check-linux.sh            # Linux container (authoritative)
#   scripts/release-check-linux.sh --host     # portable subset on this host
#
# Both modes print "RX-CHEF RELEASE CHECK: PASS" on success.

set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image=${RXCHEF_RELEASE_IMAGE:-rxchef-release-check}

if [[ "${1:-}" == "--host" ]]; then
    shift
    exec "$repo_root/scripts/release-gates.sh" host "$@"
fi

if ! docker info >/dev/null 2>&1; then
    cat >&2 <<'EOF'
error: Docker is not available, so the authoritative Linux x86_64 release
       check cannot run.

       Start Docker and re-run this script, or run the portable subset on this
       host with:

           scripts/release-check-linux.sh --host

       Host mode skips the Linux-only gates (FFI C linking, packaging,
       benchmarks) and says so in its summary.
EOF
    exit 1
fi

docker build --platform linux/amd64 \
    -f "$repo_root/docker/Dockerfile.release" \
    -t "$image" "$repo_root"

docker run --rm --platform linux/amd64 \
    -v "$repo_root:/src" \
    -v rxchef-release-cargo-git:/root/.cargo/git \
    -v rxchef-release-cargo-registry:/root/.cargo/registry \
    -v rxchef-release-target:/tmp/rxchef-target \
    "$image" "$@"
