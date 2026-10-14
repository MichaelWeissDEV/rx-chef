#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
npm --prefix apps/web ci
npm --prefix apps/web run contracts:check
npm --prefix apps/web test
npm --prefix apps/web run lint
npm --prefix apps/web run build
cargo test --locked -p rxchef-server
cargo test --locked -p rxchef-server --features web-ui
cargo build --locked -p rxchef-server --features web-ui
npm --prefix apps/web run test:e2e
cargo test --locked -p rxchef-gui
cargo test --locked -p rxchef-tui
cargo build --locked -p rxchef-tui
case "$(uname -s)" in
  Darwin|Linux) python3 scripts/tui-graph-smoke.py target/debug/rxchef_tui --server-binary target/debug/rxchef-server ;;
  *) echo "NOT TESTED: POSIX TUI PTY gate requires macOS/Linux; use a native Windows terminal test" ;;
esac
