# Current status for v0.1.0

## Workspace Configuration
- **Version:** 0.1.0
- **Workspace Crates:** `rxchef` (lib), `rxchef_cli`, `rxchef_tui`, `rxchef_store`, `cyberchef-rust-tests`
- **Binaries:** `rxchef` (from `rxchef_cli`), though `Cargo.toml` specifies a default-run of `rxchef`.

## Features
Defined features in `rxchef`:
- `pgp` (depends on `sequoia-openpgp`)
- `jsonata` (depends on `jaq-core`, `jaq-std`)
- `tesseract` (depends on `tesseract-rs`, `leptonica-sys`)
- `disassembly` (depends on `capstone`)
- `yara` (depends on `yara-x`)
- `full` (enables all optional operation backends)

## Build gates

The required release gate is the local, reproducible Linux x86_64 Docker pipeline:

```console
./scripts/release-check-linux.sh
```

The original-tree Linux baseline passed. The current consolidated tree has not
completed the final Docker gate and is therefore not yet release-verified.
GitHub Actions is not part of this release process. macOS and Windows are also
not release-verified by this version.

## Verification

- 478 operations are registered and inventoried.
- The default operation suite contains 1,704 executed tests after moving
  Capstone and YARA out of the default dependency graph; all-feature tests run
  their feature-specific cases.
- `verification/operations.json` is the explicit evidence source.
  `docs/_generated/operation-quality.json` records only reviewed KAT,
  differential, property, fuzz, benchmark, docs, status, and parity claims.
- Optional-feature availability is checked separately from correctness/parity.

## Known limitations
- Entries marked `feature_disabled` in a minimal build require the optional
  feature named in the feature matrix; they refuse to return placeholder results.
- OCR requires the optional `tesseract` feature and system Tesseract/Leptonica libraries.
- PGP is implemented and tested behind `--features pgp`.
- The locked dependency graph currently contains upstream-yanked `spin 0.9.8`;
  locked builds remain reproducible, but dependency review/update is required
  before publishing a final release.

## Measurement provenance

The local consolidation baseline and its exact commit/toolchain are recorded in
`development/baseline.md`. Benchmark artifacts include their own environment.
