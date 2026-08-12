# Installation

## Requirements

- Rust toolchain with Cargo (edition 2021)
- a C compiler for native dependencies such as YARA and Capstone
- optional system libraries when enabling OCR or other native feature flags

## Install the CLI

From a checkout:

```console
cargo install --path crates/cli
rxchef --version
```

For development, avoid installing and run the workspace package directly:

```console
cargo run -p rxchef_cli -- --help
```

Build all default workspace targets:

```console
cargo build --workspace
```

The optimized CLI binary is `target/release/rxchef` after:

```console
cargo build --release -p rxchef_cli
```

## Use as a Rust library

For a local checkout:

```toml
[dependencies]
rxchef = { path = "../rxchef" }
```

The library supports these optional features:

| Feature | Adds |
|---|---|
| `pgp` | OpenPGP operations through Sequoia |
| `jsonata` | JSON query support through jaq |
| `tesseract` | OCR through Tesseract and Leptonica |

Enable only the features required by the embedding application.
