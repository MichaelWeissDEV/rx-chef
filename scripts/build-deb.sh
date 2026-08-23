#!/usr/bin/env bash
#
# Builds rxchef_<version>_<arch>.deb and rxchef-tui_<version>_<arch>.deb.
#
# Requires cargo-deb (cargo install cargo-deb --locked --version 3.7.0).
# Man page and shell completions come from the binary itself, so they are
# generated here before packaging rather than checked in as static assets.
#
# Usage: scripts/build-deb.sh

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

echo "==> cargo deb -p rxchef-cli"
cargo deb -p rxchef-cli --no-build

echo "==> cargo deb -p rxchef-tui"
cargo deb -p rxchef-tui --no-build

echo "==> built packages:"
find target/debian -maxdepth 1 -name '*.deb'
