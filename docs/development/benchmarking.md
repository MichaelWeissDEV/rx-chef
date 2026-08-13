# Benchmarking

Run `cargo run --release -p xtask -- bench`. Debug builds are rejected because
their timings are not representative.

Cases live in `crates/xtask/src/bench.rs`. Add a deterministic payload, call the
public runtime or engine entry point inside `run_case`, and choose iterations
that finish quickly enough for local development. Never hide setup cost inside
or outside the timed closure without documenting that decision.

Benchmarks are regression signals, not correctness tests. Add or update normal
tests first, then benchmark the behavior that matters. See the
[performance methodology](../performance/methodology.md).
