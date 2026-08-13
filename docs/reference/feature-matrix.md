# Feature matrix

This page gives a high-level view of the project’s platform and feature coverage.

## Cargo features

| Feature | Operation/dependency | Default | Notes |
|---|---|---:|---|
| `jsonata` | Jsonata Query via `jaq` | no | Pure Rust; enables real query execution. |
| `pgp` | Six OpenPGP operations via Sequoia | no | RustCrypto backend; key generation, encrypt/decrypt, sign/verify, and combined round trips are tested. |
| `tesseract` | Optical Character Recognition | no | Links a system Tesseract/Leptonica installation through `pkg-config`. |

All feature combinations compile in CI-compatible environments with the OCR
system packages installed:

```console
cargo check --workspace --all-targets --all-features
```

The core library, CLI pipelines, recipes, Magic, Scan, TUI, machine-readable
registry, and stdio server are available without optional features. The runtime
`broken` field means that an optional backend is unavailable in the current
build. With `--all-features` and the documented native OCR libraries installed,
all 478 registry entries are available; CI asserts this invariant.

## Related pages

- [Operation matrix](operation-matrix.md)
- [Reference index](operations.md)
- [Project overview](../project/overview.md)
