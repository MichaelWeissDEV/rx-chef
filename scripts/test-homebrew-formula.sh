#!/usr/bin/env bash
#
# Exercises a Homebrew formula the way an end user would: audit, install
# from the real released archive, and a functional smoke test (not just
# `--version`).
#
# This needs a real GitHub release to exist, because the formula's `url`
# points at that release's platform archive — it cannot be exercised before
# the first real tag is published. Run it against the actual tap after a
# release, or locally against a formula regenerated with
# `dist build --artifacts=global` from a tag that has already been released.
#
# Usage: scripts/test-homebrew-formula.sh <path-to-formula.rb> <binary-name>
#
#   scripts/test-homebrew-formula.sh target/distrib/rxchef.rb rxchef
#   scripts/test-homebrew-formula.sh target/distrib/rxchef-tui.rb rxchef_tui

set -euo pipefail

formula=${1:?usage: $0 <path-to-formula.rb> <binary-name>}
binary=${2:?usage: $0 <path-to-formula.rb> <binary-name>}
script_dir=$(cd "$(dirname "$0")" && pwd)
formula_name=$(basename "$formula" .rb)
case "$binary" in
    rxchef|rxchef_tui) ;;
    *) echo "error: unsupported binary: $binary" >&2; exit 2 ;;
esac

echo "==> brew style"
brew style "$formula"

echo "==> brew audit"
brew audit --formula "$formula" --except=github_repository_link

echo "==> brew install --build-from-source"
brew install --build-from-source "$formula"

# Resolve the installed keg explicitly so another PATH entry cannot pass this test.
trap 'brew uninstall "$formula_name"' EXIT
installed_binary="$(brew --prefix "$formula_name")/bin/$binary"
echo "==> functional smoke test of $installed_binary"
case "$binary" in
    rxchef)
        "$installed_binary" --version
        printf 'hello' | "$installed_binary" run 'To Base64' | grep -qx 'aGVsbG8=' \
            || { echo "error: unexpected Base64 output" >&2; exit 1; }
        ;;
    rxchef_tui)
        "$installed_binary" --help | grep -q -- '--graph'
        python3 "$script_dir/tui-graph-smoke.py" "$installed_binary"
        ;;
esac

echo "==> brew uninstall"
brew uninstall "$formula_name"
trap - EXIT

echo "HOMEBREW FORMULA TEST: PASS"
