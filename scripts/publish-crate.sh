#!/usr/bin/env bash
#
# Idempotently publishes one crate: skips if this exact package@version is
# already on crates.io (so a re-run of a partially-failed release workflow
# doesn't error out on crates a previous attempt already published), then
# waits for the registry to actually resolve the new version before
# returning, so a dependent crate's publish step isn't racing propagation.
#
# Usage: scripts/publish-crate.sh <package-name> [--dry-run]
#
# Requires CARGO_REGISTRY_TOKEN to be set (bootstrap token or a trusted-
# publishing OIDC token obtained by the caller) unless --dry-run is passed.

set -euo pipefail

package=${1:?usage: $0 <package-name> [--dry-run]}
dry_run=false
if [[ "${2:-}" == "--dry-run" ]]; then
    dry_run=true
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

version=$(
    cargo metadata --locked --format-version 1 --no-deps 2>/dev/null \
        | python3 -c "
import json, sys
data = json.load(sys.stdin)
versions = {p['name']: p['version'] for p in data['packages']}
print(versions['$package'])
"
)

echo "==> $package $version"

already_published() {
    local status
    status=$(curl -s -o /dev/null -w '%{http_code}' \
        -A "rxchef-release (publish-crate.sh)" \
        "https://crates.io/api/v1/crates/${package}/${version}")
    [[ "$status" == "200" ]]
}

if already_published; then
    echo "SKIP: ${package} ${version} is already published"
    exit 0
fi

if [[ "$dry_run" == true ]]; then
    echo "DRY RUN: would publish ${package} ${version}"
    cargo publish --locked -p "$package" --dry-run --allow-dirty
    exit 0
fi

echo "publishing ${package} ${version}..."
cargo publish --locked -p "$package"

echo "waiting for ${package} ${version} to propagate to the registry index..."
attempt=0
max_attempts=30
until already_published; do
    attempt=$((attempt + 1))
    if ((attempt >= max_attempts)); then
        echo "error: ${package} ${version} did not appear on crates.io after ${max_attempts} attempts" >&2
        exit 1
    fi
    sleep 10
done
echo "${package} ${version} is live"
