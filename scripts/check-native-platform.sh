#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

echo "==> Native platform: $(uname -s) $(uname -m)"
rustc --version
cargo --version

npm --prefix apps/web ci
npm --prefix apps/web run contracts:check
npm --prefix apps/web run build
npm --prefix apps/web test
npm --prefix apps/web run lint

cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo test --locked --workspace
cargo test --locked -p rxchef-client --features remote
cargo clippy --locked --workspace --all-targets -- \
    -D clippy::correctness -D clippy::suspicious
cargo run --locked --package xtask -- check-registry
cargo run --locked --package xtask -- audit-operations
cargo run --locked --package xtask -- docs --check
cargo run --locked --example generate_operation_docs -- --check
mkdocs build --strict
cargo build --locked --release -p rxchef-cli -p rxchef-tui -p rxchef -p rxchef-server

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        rxchef_binary=target/release/rxchef.exe
        python_command=python
        ;;
    *)
        rxchef_binary=target/release/rxchef
        python_command=python3
        ;;
esac

"$rxchef_binary" --version
"$rxchef_binary" operations --all --json >/dev/null
"$python_command" docker/server-smoke.py "$rxchef_binary"
case "$(uname -s)" in
    Darwin|Linux)
        "$python_command" scripts/tui-graph-smoke.py target/release/rxchef_tui --server-binary target/release/rxchef-server
        ;;
    *) echo "NOT TESTED: POSIX TUI PTY smoke; native Windows console testing required" ;;
esac

echo "==> Native platform gate passed"
