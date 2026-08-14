# Benchmarking

Run `cargo run --release -p xtask -- bench`. Debug builds are rejected because
their timings are not representative.

For a machine-readable artifact use `cargo xtask bench-docs --quick` or
`cargo xtask bench-docs --full`. The command relaunches itself with the release
profile and writes environment-labelled JSON below `target/benchmarks/` so a
benchmark never dirties the repository. Set `RXCHEF_BENCH_OUTPUT` to an explicit
path when an external performance job needs to collect the file. Results are
reference measurements, hardware-dependent, and never runtime guarantees.

Cases live in `crates/xtask/src/bench.rs`. Add a deterministic payload, call the
public runtime or engine entry point inside `run_case`, and choose iterations
that finish quickly enough for local development. Never hide setup cost inside
or outside the timed closure without documenting that decision.

Benchmarks are regression signals, not correctness tests. Add or update normal
tests first, then benchmark the behavior that matters. See the
[performance methodology](../performance/methodology.md).

Example with an externally collected artifact:

```console
RXCHEF_BENCH_OUTPUT=/tmp/rxchef-bench.json \
  cargo xtask bench-docs --full
jq '.environment, .results[] | {name, median_ns, throughput_mib_s}' \
  /tmp/rxchef-bench.json
```
