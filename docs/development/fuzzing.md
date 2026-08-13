# Fuzzing

v0.0.1 does not ship a committed fuzz target. The normal suite covers operation
vectors, invalid inputs, pipelines, CLI parsing, and the stdio protocol. For a
local parser campaign, create a temporary `cargo-fuzz` target that calls the
public operation or recipe parser with arbitrary bytes and treats panics as
failures.

Good targets are archive/protocol parsers, recipe JSON/YAML, compact pipeline
syntax, image decoders, and binary format operations. Bound recursion, output
size, and decompression to prevent resource exhaustion from masking logic bugs.
Promote every minimized crash into a deterministic regression test before fixing
it. Do not claim fuzz coverage in release notes unless the corpus, duration,
toolchain, and target are recorded.
