# Performance results

No hardware-independent numbers are published for v0.0.1. Throughput depends on
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
