#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

"$repo_root/scripts/release-check-linux.sh"
"$repo_root/scripts/check-windows-cross.sh"

case "$(uname -s)" in
    Darwin|MINGW*|MSYS*|CYGWIN*)
        "$repo_root/scripts/check-native-platform.sh"
        ;;
    *)
        echo "==> Native macOS/Windows execution is run on its corresponding host"
        echo "    scripts/check-native-platform.sh"
        ;;
esac
