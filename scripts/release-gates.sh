#!/usr/bin/env bash
#
# Portable rx-chef release gates.
#
# This is the single definition of what "release ready" means. It is executed
# both inside the Linux release container (via docker/release-entrypoint.sh)
# and directly on a developer host (via scripts/release-check-linux.sh --host).
#
# Gates that genuinely cannot run outside the Linux x86_64 container are
# skipped explicitly and reported, never silently dropped: the final summary
# lists every skipped gate, and the PASS line is only printed when no gate
# failed.
#
# Usage: release-gates.sh <linux|host>

# Deliberately not `set -e`: every gate runs so the summary lists all failures
# rather than stopping at the first one. The exit status comes from the
# collected `failed` array at the end.
set -uo pipefail

mode=${1:-host}
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

: "${CARGO_TARGET_DIR:=$repo_root/target}"
export CARGO_TARGET_DIR
export RXCHEF_HOME="${RXCHEF_HOME:-$(mktemp -d)}"
mkdir -p "$RXCHEF_HOME"

skipped=()
failed=()
not_tested=()

run_gate() {
    local label=$1
    shift
    printf '==> %s\n' "$label"
    if "$@"; then
        return 0
    fi
    printf '!!! FAILED: %s\n' "$label" >&2
    failed+=("$label")
    # Always report success so later gates still run; the summary decides the
    # overall exit status.
    return 0
}

skip_gate() {
    local label=$1
    local reason=$2
    printf '==> %s [SKIPPED: %s]\n' "$label" "$reason"
    skipped+=("$label — $reason")
}

not_tested_gate() {
    printf '==> %s [NOT TESTED: %s]\n' "$1" "$2"
    not_tested+=("$1 — $2")
}

# --- Toolchain and lockfile ------------------------------------------------

run_gate "cargo metadata (locked)" \
    cargo metadata --locked --format-version 1 --no-deps >/dev/null
run_gate "release version regression" python3 scripts/test-release-version.py
run_gate "distribution regression" python3 scripts/test-distributions.py
run_gate "crate publication regression" python3 scripts/test-publish-crate.py
run_gate "RustSec dependency audit" cargo audit --json

# Web source generation precedes all-features Rust builds (embedded assets).
run_gate "frontend clean install" npm --prefix apps/web ci
run_gate "frontend dependency audit" npm --prefix apps/web audit --json
run_gate "wire contracts" npm --prefix apps/web run contracts:check
run_gate "frontend typecheck" npm --prefix apps/web run typecheck
run_gate "frontend unit tests" npm --prefix apps/web test
run_gate "frontend lint" npm --prefix apps/web run lint
run_gate "frontend build" npm --prefix apps/web run build
if [[ "$mode" == "linux" || "$mode" == "linux-arm" ]]; then
    run_gate "browser runtime" npm --prefix apps/web exec -- playwright install --with-deps chromium
fi
run_gate "browser E2E" npm --prefix apps/web run test:e2e

# --- Static checks ---------------------------------------------------------

run_gate "format" cargo fmt --all -- --check
run_gate "workspace check" cargo check --locked --workspace --all-targets
run_gate "clippy (correctness, suspicious)" \
    cargo clippy --locked --workspace --all-targets -- \
    -D clippy::correctness -D clippy::suspicious

# The default feature set leaves the optional integrations unlinted. Running
# clippy again with every feature enabled is the only way the PGP, JSONata,
# OCR, YARA and disassembly code paths get checked at all. This runs in both
# modes for the same reason the all-features check and tests below do: the
# optional native dependencies build on the supported developer hosts.
run_gate "clippy all-features (correctness, suspicious)" \
    cargo clippy --locked --workspace --all-targets --all-features -- \
    -D clippy::correctness -D clippy::suspicious

# --- Tests -----------------------------------------------------------------

run_gate "workspace tests" cargo test --locked --workspace
run_gate "remote client tests" cargo test --locked -p rxchef-client --features remote
run_gate "web-enabled server tests" cargo test --locked -p rxchef-server --features web-ui
run_gate "all-features check" cargo check --locked --workspace --all-targets --all-features
run_gate "all-features tests" cargo test --locked --workspace --all-features

# --- Project-specific consistency gates ------------------------------------

run_gate "registry consistency" cargo run --locked --package xtask -- check-registry
run_gate "operation audit" cargo run --locked --package xtask -- audit-operations
run_gate "version consistency" cargo run --locked --package xtask -- verify-version
run_gate "generated operation docs" cargo run --locked --package xtask -- docs --check
run_gate "generated operation reference" \
    cargo run --locked --example generate_operation_docs -- --check
run_gate "fuzz targets compile" \
    cargo check --locked --manifest-path fuzz/Cargo.toml --bins

run_gate "UI acceptance rules" python3 scripts/test-ui-acceptance.py

if command -v mkdocs >/dev/null 2>&1; then
    run_gate "MkDocs strict" mkdocs build --strict
else
    skip_gate "MkDocs strict" "mkdocs is not installed on this host"
fi

# --- Release artefacts -----------------------------------------------------

run_gate "release CLI" cargo build --locked --release -p rxchef-cli
run_gate "release TUI" cargo build --locked --release -p rxchef-tui
run_gate "release library/FFI" cargo build --locked --release -p rxchef
run_gate "release server" cargo build --locked --release -p rxchef-server
case "$(uname -s)" in
    Darwin|Linux) run_gate "real terminal graph smoke" python3 scripts/tui-graph-smoke.py "$CARGO_TARGET_DIR/release/rxchef_tui" --server-binary "$CARGO_TARGET_DIR/release/rxchef-server" ;;
    *) skip_gate "real terminal graph smoke" "POSIX PTY test requires macOS/Linux; native Windows console tests required" ;;
esac

rxchef="$CARGO_TARGET_DIR/release/rxchef"

# --- CLI smoke tests -------------------------------------------------------

run_gate "CLI version" "$rxchef" --version
run_gate "CLI help" bash -c '"$1" --help >/dev/null' _ "$rxchef"

# The expected operation count is derived from the audit output rather than
# hardcoded, so adding an operation does not silently break this gate.
expected_operations=$(
    python3 -c "import json;print(json.load(open('docs/_generated/operation-quality.json'))['operation_count'])" \
        2>/dev/null || echo 0
)
run_gate "operation catalog (${expected_operations} operations)" bash -c \
    '[[ "$("$1" operations --all --json | python3 -c "import json,sys;print(len(json.load(sys.stdin)))")" == "$2" ]]' \
    _ "$rxchef" "$expected_operations"

run_gate "operation describe" bash -c \
    '[[ "$("$1" operation describe "From Base64" --json | python3 -c "import json,sys;print(json.load(sys.stdin)[\"name\"])")" == "From Base64" ]]' \
    _ "$rxchef"
run_gate "functional decode" bash -c \
    '[[ "$(printf SGVsbG8= | "$1" run "From Base64")" == Hello ]]' _ "$rxchef"

# Every byte value must survive a Base64 round trip through the CLI, including
# NUL and invalid-UTF-8 sequences.
roundtrip_dir=$(mktemp -d)
printf '\000\001\002\177\200\376\377\342\202\254\000' >"$roundtrip_dir/original.bin"
run_gate "binary roundtrip" bash -c \
    'base64 <"$2/original.bin" | "$1" run "From Base64" >"$2/restored.bin" \
        && cmp "$2/original.bin" "$2/restored.bin"' \
    _ "$rxchef" "$roundtrip_dir"

run_gate "recipe smoke" bash -c \
    '[[ "$(printf Hello | "$1" bake --recipe tests/fixtures/recipes/release-smoke.yaml)" == SEVMTE8= ]]' \
    _ "$rxchef"

# Failures must exit non-zero, otherwise scripts consuming the CLI cannot tell
# success from failure. Covers an unknown operation, a rejected argument, and
# rejected input.
run_gate "unknown operation exits non-zero" bash -c \
    '! printf x | "$1" run "No Such Operation" >/dev/null 2>&1' _ "$rxchef"
run_gate "invalid argument exits non-zero" bash -c \
    '! printf x | "$1" run "GOST Hash" "bogus-algorithm" >/dev/null 2>&1' _ "$rxchef"
run_gate "invalid input exits non-zero" bash -c \
    '! printf "SGVsbG8!" | "$1" run "From Base64" "A-Za-z0-9+/=" false true >/dev/null 2>&1' \
    _ "$rxchef"

run_gate "shipped command help" python3 scripts/test-command-help.py --directory "$CARGO_TARGET_DIR/release"
run_gate "server smoke" python3 docker/server-smoke.py "$rxchef"
run_gate "executable handbook examples" python3 scripts/test-workbench-docs.py --cli "$rxchef"

# Keep variants in a fresh directory: never silently reuse or overwrite a
# previous release rehearsal. Web and headless snapshots stay distinct.
export RXCHEF_DISTRIBUTION_DIR
RXCHEF_DISTRIBUTION_DIR=$(mktemp -d "$CARGO_TARGET_DIR/rc-packages-XXXXXXXX")
for variant in cli tui headless web; do
    run_gate "package $variant" bash scripts/build-distributions.sh "$variant"
done
run_gate "archive installation/reinstallation smoke" \
    python3 scripts/smoke-distributions.py --directory "$RXCHEF_DISTRIBUTION_DIR"
case "$(uname -s)" in
    Darwin) run_gate "native desktop app/DMG bundle" npm --prefix apps/web run desktop:build -- --bundles app,dmg ;;
    Linux) run_gate "native desktop bundles" npm --prefix apps/web run desktop:build ;;
    *) not_tested_gate "native desktop bundles" "use the native Windows installer matrix" ;;
esac
run_gate "package inventory and SHA-256" python3 scripts/inventory-release-artifacts.py
if command -v dist >/dev/null 2>&1; then
    run_gate "cargo-dist generated workflow" dist generate --check
    run_gate "cargo-dist plan" dist plan
    host_target=$(rustc -vV | sed -n 's/^host: //p')
    run_gate "cargo-dist host archives" dist build --artifacts local --target "$host_target"
else
    not_tested_gate "cargo-dist plan/build" "install configured cargo-dist 0.32.0 before approval"
fi
not_tested_gate "native GUI installer acceptance" "manual GUI/dialog/offline/uninstall evidence must be reviewed separately"
not_tested_gate "other native platforms" "a host gate cannot certify other OS/architectures"

# --- Linux x86_64 only -----------------------------------------------------

if [[ "$mode" == "linux" || "$mode" == "linux-arm" ]]; then
    run_gate "FFI C compile" cc -std=c11 -Wall -Wextra -Werror -Iinclude \
        docker/ffi-smoke.c -L"$CARGO_TARGET_DIR/release" -lrxchef -o /tmp/rxchef-ffi-smoke
    run_gate "FFI C execute" \
        env LD_LIBRARY_PATH="$CARGO_TARGET_DIR/release" /tmp/rxchef-ffi-smoke
    if [[ "$mode" == "linux" ]]; then
        run_gate "quick Linux benchmarks" cargo run --locked --package xtask -- bench-docs --quick
    else
        skip_gate "quick x86_64 Linux benchmarks" "this runner is Linux aarch64; do not label its measurements as x86_64"
    fi
    run_gate "Core package" cargo package --locked -p rxchef --allow-dirty
    # Dependent crates resolve through crates.io even during package verification.
    # Before the first core publication, validate their contents here; the real
    # dependency-ordered publish verifies each crate after its dependencies exist.
    run_gate "Store package contents" cargo package --locked -p rxchef-store --allow-dirty --list >/dev/null
    not_tested_gate "dependent crate archive verification" "requires registry bootstrap; publication verifies in core/store/client/CLI/TUI order"
    run_gate "Client package contents" \
        cargo package --locked -p rxchef-client --allow-dirty --list >/dev/null
    run_gate "CLI package contents" \
        cargo package --locked -p rxchef-cli --allow-dirty --list >/dev/null
    run_gate "TUI package contents" \
        cargo package --locked -p rxchef-tui --allow-dirty --list >/dev/null
    run_gate "cargo install CLI from source" \
        cargo install --path crates/cli --root /tmp/rxchef-install --force --locked
    run_gate "installed CLI smoke" /tmp/rxchef-install/bin/rxchef --version
else
    skip_gate "FFI C compile and execute" \
        "requires the Linux x86_64 container (uses -lrxchef and LD_LIBRARY_PATH)"
    skip_gate "quick Linux benchmarks" "benchmark numbers are only recorded on Linux x86_64"
    skip_gate "cargo package and install" "release packaging is verified in the Linux container"
fi

# --- Summary ---------------------------------------------------------------

echo
echo "-----------------------------------------------------------------------"
if ((${#not_tested[@]} > 0)); then
    echo "Not tested by this gate (${#not_tested[@]}):"
    for entry in "${not_tested[@]}"; do echo "  - $entry"; done
fi
if ((${#skipped[@]} > 0)); then
    echo "Skipped gates (${#skipped[@]}):"
    for entry in "${skipped[@]}"; do
        echo "  - $entry"
    done
fi

if ((${#failed[@]} > 0)); then
    echo "Failed gates (${#failed[@]}):"
    for entry in "${failed[@]}"; do
        echo "  - $entry"
    done
    echo "RX-CHEF RELEASE CHECK: FAIL"
    exit 1
fi

if [[ "$mode" == "linux" || "$mode" == "linux-arm" ]]; then
    echo "Mode: Linux container ($(uname -m))"
else
    echo "Mode: host ($(uname -s) $(uname -m)) — Linux-only gates were skipped"
fi
echo "RX-CHEF RELEASE CHECK: PASS"
echo "Scope: checks executed above on this host only; skips and missing native installer/platform evidence still block RC approval."
