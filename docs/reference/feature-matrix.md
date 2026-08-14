# Feature matrix

This page gives a high-level view of the project’s platform and feature coverage.

## Cargo features

| Feature | Operation/dependency | Default | Notes |
|---|---|---:|---|
| `jsonata` | Jsonata Query via `jaq` | no | Pure Rust; enables real query execution. |
| `pgp` | Six OpenPGP operations via Sequoia | no | RustCrypto backend; key generation, encrypt/decrypt, sign/verify, and combined round trips are tested. |
| `tesseract` | Optical Character Recognition | no | Links a system Tesseract/Leptonica installation through `pkg-config`. |
| `disassembly` | ARM/x86 disassembly via Capstone | no | Keeps Capstone out of the normal encoding/text installation. |
| `yara` | YARA Rules via yara-x | no | Keeps the large YARA/Wasmtime dependency graph optional. |
| `full` | All optional operation backends | no | Convenience group for Linux release verification. |

All features are checked in the Linux x86_64 release container with the OCR
system packages installed:

```console
cargo check --workspace --all-targets --all-features
```

The core library, CLI pipelines, recipes, Magic, Scan, TUI, machine-readable
registry, and stdio server are available without optional features. Runtime
`availability` identifies an optional backend that is unavailable in the
current build; `implementation_status` is independent. With `--all-features`
and the documented native OCR libraries installed,
all 478 registry entries are expected to be available; the local Docker release
gate asserts this invariant.

The verified platform for this release is Linux x86_64. macOS and Windows are
not verified in this release process.

## Related pages

- [Operation matrix](operation-matrix.md)
- [Reference index](operations.md)
- [Project overview](../project/overview.md)
