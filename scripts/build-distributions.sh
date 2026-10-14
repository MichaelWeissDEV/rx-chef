#!/usr/bin/env bash
# Local packaging only: never publishes, pushes or signs.
set -euo pipefail
cd "$(dirname "$0")/.."
variant=${1:-web}
output="target/distributions/$variant"
mkdir -p "$output"
case "$variant" in
  library) cargo build --locked --release -p rxchef; cp target/release/librxchef.* "$output/" ;;
  cli) cargo build --locked --release -p rxchef-cli; cp target/release/rxchef "$output/" ;;
  tui) cargo build --locked --release -p rxchef-tui; cp target/release/rxchef_tui "$output/" ;;
  combined) cargo build --locked --release -p rxchef-cli -p rxchef-tui; cp target/release/rxchef target/release/rxchef_tui "$output/" ;;
  headless) cargo build --locked --release -p rxchef-server; cp target/release/rxchef-server "$output/" ;;
  web)
    npm --prefix apps/web ci
    npm --prefix apps/web run build
    cargo build --locked --release -p rxchef-server --features web-ui
    cp target/release/rxchef-server "$output/"
    ;;
  desktop) npm --prefix apps/web ci; npm --prefix apps/web run desktop:build; exit ;;
  *) echo 'Usage: build-distributions.sh library|cli|tui|combined|headless|web|desktop' >&2; exit 2 ;;
esac
cp LICENSE "$output/"
tar -czf "target/distributions/rxchef-$variant-$(uname -s)-$(uname -m).tar.gz" -C "$output" .
