#!/usr/bin/env bash
#
# Entrypoint for the Linux x86_64 release container.
#
# The gate list itself lives in scripts/release-gates.sh so the container and a
# developer host run exactly the same checks; this script only establishes the
# container-specific environment and asserts the platform.

set -euo pipefail

cd /src

if [[ "$(uname -m)" != "x86_64" ]]; then
    echo "release check requires Linux x86_64, found $(uname -m)" >&2
    exit 1
fi

export CARGO_TARGET_DIR=/tmp/rxchef-target
export RXCHEF_HOME=/tmp/rxchef-home
mkdir -p "$RXCHEF_HOME"

exec /src/scripts/release-gates.sh linux "$@"
