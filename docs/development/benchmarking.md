# Benchmarking

Run `cargo run --release -p xtask -- bench`. Debug builds are rejected because
their timings are not representative.

For a documentation artifact use `cargo xtask bench-docs --quick` or
`cargo xtask bench-docs --full`. The command relaunches itself with the release
profile and writes environment-labelled JSON to `benchmarks/results/`.
Linux x86_64 writes `linux-x86_64.json`; other hosts write
`host-unverified.json` and cannot overwrite Linux release evidence. Results are reference measurements,
hardware-dependent, and never runtime guarantees. Until that command has run,
the release documentation must say **Not measured**.

Cases live in `crates/xtask/src/bench.rs`. Add a deterministic payload, call the
public runtime or engine entry point inside `run_case`, and choose iterations
that finish quickly enough for local development. Never hide setup cost inside
or outside the timed closure without documenting that decision.

Benchmarks are regression signals, not correctness tests. Add or update normal
tests first, then benchmark the behavior that matters. See the
[performance methodology](../performance/methodology.md).
