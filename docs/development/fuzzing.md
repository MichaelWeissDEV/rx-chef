# Fuzzing

v0.1.0 ships three bounded `cargo-fuzz` targets under `fuzz/fuzz_targets`:
recipe/execution parsing, the Magic search, and boundary-aware streaming scan.
Compile all targets with:

```bash
cargo check --manifest-path fuzz/Cargo.toml --bins
```

Run a target with, for example,
`cargo fuzz run --fuzz-dir fuzz execution_recipe`. CI compiles the targets but
does not claim campaign duration or corpus coverage.

Future targets should cover archive/protocol parsers, compact pipeline syntax,
image decoders, and binary format operations. Bound recursion, output size, and
decompression to prevent resource exhaustion from masking logic bugs.
Promote every minimized crash into a deterministic regression test before fixing
it. Do not claim fuzz coverage in release notes unless the corpus, duration,
toolchain, and target are recorded.
