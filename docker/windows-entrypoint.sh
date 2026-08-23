#!/usr/bin/env bash
set -euo pipefail

cd /src

target=x86_64-pc-windows-gnu
export CARGO_TARGET_DIR=/tmp/rxchef-windows-target

echo "==> Windows target: ${target}"
rustc --version
cargo --version
x86_64-w64-mingw32-gcc --version | head -n 1

cargo fmt --all -- --check
cargo check --locked -p rxchef -p rxchef-store -p rxchef-cli -p rxchef-tui --target "$target"
cargo build --locked --release -p rxchef-cli -p rxchef-tui --target "$target"

test -f "$CARGO_TARGET_DIR/$target/release/rxchef.exe"
test -f "$CARGO_TARGET_DIR/$target/release/rxchef_tui.exe"

echo "==> Windows cross-compile gate passed"
