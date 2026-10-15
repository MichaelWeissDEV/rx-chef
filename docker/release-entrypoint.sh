#!/usr/bin/env bash
#
# Entrypoint for the Linux x86_64 release container.
#
# The gate list itself lives in scripts/release-gates.sh so the container and a
# developer host run exactly the same checks; this script only establishes the
# container-specific environment and asserts the platform.

set -euo pipefail

cd /src

# /src is the explicitly mounted checkout; its host UID differs from the
# container build user. Trust only this mount, never all filesystem paths.
git config --global --add safe.directory /src

case "$(uname -m)" in
    x86_64) gate_mode=linux ;;
    aarch64) gate_mode=linux-arm ;;
    *) echo "unsupported release-check architecture: $(uname -m)" >&2; exit 1 ;;
esac

export CARGO_TARGET_DIR=/tmp/rxchef-target
export RXCHEF_HOME=/tmp/rxchef-home
mkdir -p "$RXCHEF_HOME"

exec /src/scripts/release-gates.sh "$gate_mode" "$@"
