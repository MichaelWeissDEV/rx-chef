#!/usr/bin/env bash
# Publish one allowlisted crate, then wait until its exact version can be
# resolved from the crates.io sparse index before publishing dependants.
# Usage: scripts/publish-crate.sh <package-name> [--dry-run]
set -euo pipefail

if (( $# < 1 || $# > 2 )) || [[ $# == 2 && "$2" != --dry-run ]]; then
    echo "usage: $0 <package-name> [--dry-run]" >&2
    exit 2
fi
package=$1
case "$package" in
    rxchef|rxchef-store|rxchef-client|rxchef-cli|rxchef-tui) ;;
    *) echo "error: unsupported publication package: $package" >&2; exit 2 ;;
esac
dry_run=${2:-}
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"
version=$(cargo metadata --locked --format-version 1 --no-deps | python3 -c '
import json, sys
packages = json.load(sys.stdin)["packages"]
print(next(package["version"] for package in packages if package["name"] == sys.argv[1]))
' "$package")
echo "==> $package $version"

# Always verify a dry-run, even if the version is already published. It is a
# source/package verification command, never an idempotent publication skip.
if [[ "$dry_run" == --dry-run ]]; then
    cargo publish --locked -p "$package" --dry-run --allow-dirty
    exit 0
fi

probe=$(mktemp)
trap 'rm -f "$probe"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# Return 0 for an HTTP response, 2 for an ambiguous registry/network failure.
# Only 404 is proof of absence; 429/5xx/TLS/DNS failures must stop publication.
fetch_registry() {
    local url=$1
    if ! status=$(curl --silent --show-error --location --connect-timeout 10 --max-time 30 \
        --output "$probe" --write-out '%{http_code}' \
        --user-agent "rxchef-release (publish-crate.sh)" "$url"); then
        echo "error: registry request failed: $url" >&2
        return 2
    fi
    case "$status" in
        200|404) ;;
        *) echo "error: registry returned HTTP $status: $url" >&2; return 2 ;;
    esac
}

wait_for_index() {
    local attempt
    # All five allowlisted names have length >= 4 (Cargo's aa/bb/name scheme).
    local url="https://index.crates.io/${package:0:2}/${package:2:2}/$package"
    echo "waiting for $package $version in the sparse registry index..."
    for ((attempt=1; attempt<=30; attempt++)); do
        fetch_registry "$url" || return 2
        if [[ "$status" == 200 ]]; then
            if python3 - "$probe" "$version" <<'PY'
import json, pathlib, sys
try:
    entries = [json.loads(line) for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
    if not entries or any(not isinstance(entry, dict) or "vers" not in entry for entry in entries):
        raise ValueError("invalid sparse index entries")
    found = next((entry for entry in entries if entry["vers"] == sys.argv[2]), None)
    if found and found.get("yanked", False):
        raise ValueError("requested version is yanked")
except (ValueError, OSError) as error:
    print(f"error: invalid registry response: {error}", file=sys.stderr)
    sys.exit(2)
sys.exit(0 if found else 1)
PY
            then
                echo "$package $version is resolvable in the registry index"
                return 0
            else
                local result=$?
                [[ "$result" == 1 ]] || return "$result"
            fi
        fi
        if ((attempt < 30)); then sleep 10; fi
    done
    echo "error: $package $version did not reach the sparse index after 30 attempts" >&2
    return 1
}

fetch_registry "https://crates.io/api/v1/crates/$package/$version"
if [[ "$status" == 200 ]]; then
    # A successful status with an unrelated/malformed body is not proof that
    # this exact version exists, and must never silently skip verification.
    python3 - "$probe" "$version" <<'PY'
import json, pathlib, sys
response = json.loads(pathlib.Path(sys.argv[1]).read_text())
if response["version"]["num"] != sys.argv[2]:
    raise ValueError("registry returned another version")
PY
    echo "SKIP publish: $package $version already exists; verifying index propagation"
else
    : "${CARGO_REGISTRY_TOKEN:?CARGO_REGISTRY_TOKEN is required for publication}"
    echo "publishing $package $version..."
    cargo publish --locked -p "$package"
fi
wait_for_index
