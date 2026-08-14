# Performance results

No hardware-independent numbers are published for v0.1.0. Throughput depends on
CPU features, operating system, Rust version, thermal state, and optional native
libraries; a copied number would be more misleading than useful.

Generate a local baseline with:

```console
rustc --version
cargo run --release -p xtask -- bench
```

Keep the toolchain and host fixed when evaluating a change. Record the complete
output together with the commit, CPU model, operating system, and whether the
machine was on battery power. The [methodology](methodology.md) explains the
reported statistics and limitations.

The machine-readable artifact is `benchmarks/results/linux-x86_64.json` only
after the Linux release harness runs. Measurements made on another host are
stored as `benchmarks/results/host-unverified.json` and are not Linux release
evidence.
