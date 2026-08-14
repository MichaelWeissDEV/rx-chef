#!/usr/bin/env bash
set -euo pipefail

cd /src

if [[ "$(uname -m)" != "x86_64" ]]; then
    echo "release check requires Linux x86_64, found $(uname -m)" >&2
    exit 1
fi

export CARGO_TARGET_DIR=/tmp/rxchef-target
export RXCHEF_HOME=/tmp/rxchef-home
mkdir -p "$RXCHEF_HOME"

run_gate() {
    local label=$1
    shift
    echo "==> ${label}"
    "$@"
}

run_gate "cargo metadata" cargo metadata --locked --format-version 1 --no-deps >/tmp/rxchef-metadata.json
run_gate "format" cargo fmt --all -- --check
run_gate "workspace check" cargo check --locked --workspace --all-targets
run_gate "workspace build" cargo build --locked --workspace --all-targets
run_gate "clippy" cargo clippy --locked --workspace --all-targets -- \
    -D clippy::correctness -D clippy::suspicious
run_gate "workspace tests" cargo test --locked --workspace
run_gate "all-features check" cargo check --locked --workspace --all-targets --all-features
run_gate "all-features tests" cargo test --locked --workspace --all-features
run_gate "registry" cargo run --locked --package xtask -- check-registry
run_gate "operation audit" cargo run --locked --package xtask -- audit-operations
run_gate "generated operation docs" cargo run --locked --package xtask -- docs --check
run_gate "generated operation reference" cargo run --locked --example generate_operation_docs -- --check
run_gate "fuzz targets compile" cargo check --locked --manifest-path fuzz/Cargo.toml --bins
run_gate "MkDocs strict" mkdocs build --strict
run_gate "release CLI" cargo build --locked --release -p rxchef_cli
run_gate "release TUI" cargo build --locked --release -p rxchef_tui
run_gate "release library/FFI" cargo build --locked --release -p rxchef

rxchef="$CARGO_TARGET_DIR/release/rxchef"
run_gate "CLI version" "$rxchef" --version
run_gate "CLI help" bash -c '"$1" --help >/dev/null' _ "$rxchef"
run_gate "operation catalog" bash -c '"$1" operations --all --json | jq -e "length == 478" >/dev/null' _ "$rxchef"
run_gate "operation describe" bash -c '"$1" operation describe "From Base64" --json | jq -e ".name == \"From Base64\"" >/dev/null' _ "$rxchef"
run_gate "functional decode" bash -c '[[ "$(printf SGVsbG8= | "$1" run "From Base64")" == Hello ]]' _ "$rxchef"

roundtrip_dir=$(mktemp -d)
printf '\000\001\002\177\200\376\377\342\202\254\000' > "$roundtrip_dir/original.bin"
base64 < "$roundtrip_dir/original.bin" | "$rxchef" run "From Base64" > "$roundtrip_dir/restored.bin"
run_gate "binary roundtrip" cmp "$roundtrip_dir/original.bin" "$roundtrip_dir/restored.bin"

run_gate "recipe smoke" bash -c \
    '[[ "$(printf Hello | "$1" bake --recipe tests/fixtures/recipes/release-smoke.yaml)" == SEVMTE8= ]]' \
    _ "$rxchef"
run_gate "server smoke" python3 docker/server-smoke.py "$rxchef"
run_gate "FFI C compile" cc -std=c11 -Wall -Wextra -Werror -Iinclude \
    docker/ffi-smoke.c -L"$CARGO_TARGET_DIR/release" -lrxchef -o /tmp/rxchef-ffi-smoke
run_gate "FFI C execute" env LD_LIBRARY_PATH="$CARGO_TARGET_DIR/release" /tmp/rxchef-ffi-smoke

run_gate "quick Linux benchmarks" cargo run --locked --package xtask -- bench-docs --quick

run_gate "Core package" cargo package --locked -p rxchef --allow-dirty
run_gate "Store package" cargo package --locked -p rxchef_store --allow-dirty
run_gate "CLI package contents" cargo package --locked -p rxchef_cli --allow-dirty --list \
    >/tmp/rxchef-cli-package-files.txt
run_gate "TUI package contents" cargo package --locked -p rxchef_tui --allow-dirty --list \
    >/tmp/rxchef-tui-package-files.txt
run_gate "cargo install CLI from source" cargo install --path crates/cli \
    --root /tmp/rxchef-install --force --locked
run_gate "installed CLI smoke" /tmp/rxchef-install/bin/rxchef --version

echo "==> Linux x86_64 release verification passed"
