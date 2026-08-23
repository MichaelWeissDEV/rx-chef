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

echo "==> brew style"
brew style "$formula"

echo "==> brew audit"
brew audit --formula "$formula" --except=github_repository_link

echo "==> brew install --build-from-source"
brew install --build-from-source "$formula"

echo "==> functional smoke test (not just --version)"
"$binary" --version
printf 'hello' | "$binary" run to_base64 | grep -qx 'aGVsbG8=' \
    || { echo "error: unexpected output from '$binary run to_base64'" >&2; exit 1; }

echo "==> brew uninstall"
brew uninstall "$(basename "$formula" .rb)"

echo "HOMEBREW FORMULA TEST: PASS"
