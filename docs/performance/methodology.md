# Performance methodology

The repository includes a deterministic, dependency-light throughput harness.
Run it only in release mode:

```console
cargo run --release -p xtask -- bench
```

Each case warms up first, then records repeated wall-clock samples and reports
median, median absolute deviation, minimum, maximum, and MiB/s. Inputs are
deterministically generated so runs are reproducible. Representative codec,
hash, cipher, compression, Magic, and Scan paths are included.

The harness measures in-process execution. It deliberately excludes CLI startup,
filesystem and pipe I/O, allocator profiling, cross-machine comparisons, and
claims of statistical significance. Compare results only on the same machine,
toolchain, power profile, and build configuration.
