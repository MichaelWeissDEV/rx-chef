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

## Build gates

The repository defines build, test, all-feature, formatting, Clippy, generated
artifact, documentation, install-smoke, Windows/macOS/Linux, and informational
coverage jobs. Their live result is shown by the CI badge; this static page does
not claim that a future or currently running workflow passed.

## Verification

- 478 operations are registered and inventoried.
- The operation suite contains 1,750 executed tests and no ignored tests in the
  release-consolidation measurement.
- `docs/_generated/operation-quality.json` records KAT, differential, property,
  fuzz, benchmark, docs, status, and parity evidence without upgrading unknowns.
- Optional-feature availability is checked separately from correctness/parity.

## Known limitations
- Entries marked `broken` in a minimal build require the optional feature named in the feature matrix; they refuse to return placeholder results.
- OCR requires the optional `tesseract` feature and system Tesseract/Leptonica libraries.
- PGP is implemented and tested behind `--features pgp`.
- The locked dependency graph currently contains upstream-yanked `spin 0.9.8`;
  locked builds remain reproducible, but dependency review/update is required
  before publishing a final release.

## Measurement provenance

The local consolidation baseline and its exact commit/toolchain are recorded in
`development/baseline.md`. Benchmark artifacts include their own environment.
