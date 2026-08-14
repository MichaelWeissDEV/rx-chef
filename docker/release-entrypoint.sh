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

write_environment_report() {
    local destination=$1
    local test_summary=${2:-"not run"}
    local all_features_summary=${3:-"not run"}
    . /etc/os-release
    cat > "$destination" <<EOF
# Linux release verification

- Commit: \`$(git rev-parse HEAD)\`
- Timestamp: \`$(date --utc +%Y-%m-%dT%H:%M:%SZ)\`
- Docker base: \`debian:bookworm-slim\`
- Distribution: \`${PRETTY_NAME}\`
- Architecture: \`$(uname -m)\`
- Kernel: \`$(uname -sr)\`
- Rust: \`$(rustc --version)\`
- Cargo: \`$(cargo --version)\`
- Default tests: ${test_summary}
- All-features tests: ${all_features_summary}
EOF
}

if [[ "${1:-}" == "baseline" ]]; then
    baseline_tmp=$(mktemp)
    {
        run_gate "cargo metadata" bash -c \
            'cargo metadata --format-version 1 --no-deps >/tmp/rxchef-metadata.json'
        run_gate "format" cargo fmt --all -- --check
        run_gate "check" cargo check --workspace --all-targets
        run_gate "build" cargo build --workspace --all-targets
        run_gate "tests" cargo test --workspace
        run_gate "clippy" cargo clippy --workspace --all-targets -- \
            -D clippy::correctness -D clippy::suspicious
        run_gate "all-features check" cargo check --workspace --all-targets --all-features
        run_gate "all-features tests" cargo test --workspace --all-features
    } 2>&1 | tee "$baseline_tmp"
    write_environment_report docs/development/linux-baseline.md \
        "passed (see command list below)" "passed"
    cat >> docs/development/linux-baseline.md <<'EOF'

## Results

| Gate | Result |
|---|---|
| Cargo metadata | pass |
| rustfmt | pass |
| Workspace check | pass |
| Workspace build | pass |
| Workspace tests | pass |
| Clippy correctness/suspicious | pass |
| All-features check | pass |
| All-features tests | pass |

## Known failures

The first image revision failed before compiling the workspace because the
native `fontconfig.pc` dependency was missing. Adding Debian's
`libfontconfig1-dev` and `libfreetype6-dev` packages resolved that environment
failure. There are no remaining failures in the commands above.

This baseline is Linux x86_64 only and makes no macOS, Windows, remote-CI,
publication, or long-running fuzzing claim.
EOF
    exit 0
fi

test_log=$(mktemp)
all_features_log=$(mktemp)

run_gate "cargo metadata" cargo metadata --format-version 1 --no-deps >/tmp/rxchef-metadata.json
run_gate "format" cargo fmt --all -- --check
run_gate "workspace check" cargo check --workspace --all-targets
run_gate "workspace build" cargo build --workspace --all-targets
run_gate "clippy" cargo clippy --workspace --all-targets -- \
    -D clippy::correctness -D clippy::suspicious
run_gate "workspace tests" bash -c 'set -o pipefail; cargo test --workspace 2>&1 | tee "$1"' _ "$test_log"
run_gate "all-features check" cargo check --workspace --all-targets --all-features
run_gate "all-features tests" bash -c 'set -o pipefail; cargo test --workspace --all-features 2>&1 | tee "$1"' _ "$all_features_log"
run_gate "registry" cargo xtask check-registry
run_gate "operation audit" cargo xtask audit-operations
run_gate "generated operation docs" cargo xtask docs --check
run_gate "generated operation reference" cargo run --example generate_operation_docs -- --check
run_gate "fuzz targets compile" cargo check --manifest-path fuzz/Cargo.toml --bins
run_gate "MkDocs strict" mkdocs build --strict
run_gate "release CLI" cargo build --release -p rxchef_cli
run_gate "release TUI" cargo build --release -p rxchef_tui
run_gate "release library/FFI" cargo build --release -p rxchef

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

run_gate "quick Linux benchmarks" cargo xtask bench-docs --quick

run_gate "Core package" cargo package -p rxchef --allow-dirty
run_gate "Store package" cargo package -p rxchef_store --allow-dirty
run_gate "CLI package contents" cargo package -p rxchef_cli --allow-dirty --list \
    >/tmp/rxchef-cli-package-files.txt
run_gate "TUI package contents" cargo package -p rxchef_tui --allow-dirty --list \
    >/tmp/rxchef-tui-package-files.txt
run_gate "cargo install CLI from source" cargo install --path crates/cli \
    --root /tmp/rxchef-install --force --locked
run_gate "installed CLI smoke" /tmp/rxchef-install/bin/rxchef --version

test_summary=$(python3 docker/summarize-tests.py "$test_log")
all_features_summary=$(python3 docker/summarize-tests.py "$all_features_log")
write_environment_report docs/development/final-linux-release-report.md \
    "$test_summary" "$all_features_summary"
python3 docker/append-linux-report.py docs/development/final-linux-release-report.md \
    "$CARGO_TARGET_DIR/release/rxchef" "$test_summary" "$all_features_summary"

echo "==> Linux x86_64 release verification passed"
