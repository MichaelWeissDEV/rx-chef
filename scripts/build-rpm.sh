#!/usr/bin/env bash
#
# Builds rxchef-<version>-1.<arch>.rpm and rxchef-tui-<version>-1.<arch>.rpm.
#
# Requires cargo-generate-rpm (cargo install cargo-generate-rpm --locked
# --version 0.21.0). Reuses the same generated man page and shell
# completions as scripts/build-deb.sh.
#
# Usage: scripts/build-rpm.sh

set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

echo "==> building release binaries"
cargo build --release --locked -p rxchef-cli -p rxchef-tui

generated="target/release/generated"
rm -rf "$generated"
mkdir -p "$generated/man" "$generated/completions"

echo "==> generating man page and shell completions"
target/release/rxchef manpage --output "$generated/man/rxchef.1"
gzip -9 -f "$generated/man/rxchef.1"
target/release/rxchef completions bash > "$generated/completions/rxchef.bash"
target/release/rxchef completions fish > "$generated/completions/rxchef.fish"
target/release/rxchef completions zsh > "$generated/completions/_rxchef"

echo "==> cargo generate-rpm (rxchef-cli)"
cargo generate-rpm -p crates/cli

echo "==> cargo generate-rpm (rxchef-tui)"
cargo generate-rpm -p crates/tui

echo "==> built packages:"
find target/generate-rpm -maxdepth 1 -name '*.rpm'
