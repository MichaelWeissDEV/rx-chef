# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Evidence provenance tracking for the operation audit: an optional
  `evidence_provenance` array per operation in `verification/operations.json`
  records where a specific evidence bucket's expected values actually came
  from (published standard, upstream fixture, differential run against
  CyberChef's own code, independent implementation, vs. self-generated
  regression/roundtrip/property coverage that cannot alone prove
  correctness). The audit validates the recorded type and required fields
  and now reports "with typed provenance" and "independent evidence (typed)"
  counts separately from the existing correctness/parity counts, without
  retroactively changing what "478/478 verified" means. See
  [What "verified" means](https://rx-chef.readthedocs.io/en/latest/reference/verification/)
  for the full model.
- Migrated evidence provenance for 394/478 operations from the differential
  fixture's existing (and already required, already hand-checked)
  `reference_source` field: 397/478 operations now have declared
  differential evidence (up from 266 — 135 operations had real, passing
  differential-fixture coverage that was simply never declared in
  `verification/operations.json`), and all 171 operations claiming exact
  CyberChef parity now carry a typed independent-evidence record. The
  remaining 84 operations' evidence provenance is not yet classified and is
  reported as such, not guessed.

### Fixed

- GOST Key Wrap/Unwrap no longer silently discard User Key Material (UKM) or
  substitute a placeholder ECB-encrypt-and-truncated-CBC-MAC construction.
  They now implement RFC 4357 ("NO" and CryptoPro "CP" key wrapping) with a
  GOST R 34.13-2015 CMAC-style checksum, validated against the vendored
  reference implementation CyberChef itself calls
  (`gchq/CyberChef@b92501e`, `src/core/vendor/gost/gostCipher.mjs`).
  CryptoPro wrapping for 128-bit ciphers (Kuznyechik) and SignalCom ("SC")
  wrapping now fail with an explicit, documented "not supported" error
  instead of returning a value the reference implementation cannot itself
  produce.
- GOST Sign/Verify switched from a hand-rolled single-block CBC-MAC (whose
  test coverage was, in effect, a raw block-cipher known-answer test) to the
  same real GOST R 34.13-2015 MAC construction, with reference vectors that
  exercise empty, partial, and multi-block messages.
- "GOST 28147 (1989)" is now explicitly documented, in both operations, as
  an alias for GOST R 34.12 (Magma, 2015) rather than a silent
  approximation; the original round-reduced 1989 MAC construction and
  selectable S-boxes remain unimplemented and undocumented as verified.

## [0.1.0] - 2026-08-13

### Added

- Stable `rxchef::catalog` and `rxchef::execute` library entry points.
- Explicit project initialization/discovery, `RXCHEF_HOME`, secret variables,
  atomic store writes, bounded History, shell completions, and manpage output.
- Deterministic registry generation/checking, operation quality audit, complete
  generated operation pages, and environment-labelled benchmark artifacts.
- Configurable JSONL request limits and a reproducible local Linux x86_64
  Docker release check.
- Explicit argument kinds, requirements, choices, bounds, and sensitivity for
  every registered operation, plus a conservative verification manifest.
- Bounded Magic candidate/byte budgets, bounded Scanner findings/tokens, and
  stable CLI failure classes with exit codes 2 through 6.

### Changed

- CLI, TUI, Rust API, Magic, and stdio integrations now use the shared recipe
  execution engine and exact binary output contract.
- Unprefixed operation arguments are text; binary values require explicit
  `hex:`, `bytes:`, or `base64:` prefixes.
- The C ABI is explicitly experimental and ships with `include/rxchef.h`.
- Capstone, yara-x, OCR, PGP, and JSON query backends are optional feature
  groups; unavailable operations remain discoverable with an explicit reason.

### Fixed

- HMAC literal-key ambiguity, invalid HASSH hex acceptance, Unicode preview
  slicing, scope-crossing pipeline mutations, recipe-load side effects, and
  unsafe History preview replay.
- Removed all ignored operation tests and tautological success-or-error tests.
- Removed GitHub build/test CI workflows from the current release definition;
  macOS and Windows are not claimed as verified.

## [0.0.1] - 2026-08-13

### Added
- Rust library, terminal CLI, TUI, operation registry, pipelines, recipes, and JSONL stdio server.
- Machine-readable `operations`, `operation describe`, `run`, and `bake` interfaces.
- Read the Docs documentation, generated operation reference, CI, and benchmark harness.
- Persistent JSONL/JSON-RPC `serve --stdio` transport for editor and plugin clients.
- Shared recipe flow control for Fork/Merge, Subsection, Register expansion, labels, and bounded jumps across library, CLI, projects, and stdio clients.
- Real JSON query, XPath, QR-code generation/parsing, MD6, SNEFRU-256, SHA-0, HAS-160, SM2, GOST wrap/verify, AMF0/AMF3, Protobuf schemas, JavaScript AST parsing, hex-density charts, and all advertised AES modes.
- Sequoia-backed OpenPGP key generation, encryption/decryption, signing/verification, and combined workflows behind the `pgp` feature.
- XSalsa8/12/20, both Rabbit byte orders, portable bitmap text rendering, arbitrary-angle image rotation, and fenced-code Markdown highlighting.

### Fixed
- Optional `jsonata` and `pgp` features now compile against their current dependencies.
- SNEFRU now computes standardized SNEFRU-256 instead of emitting a falsely labelled SHA-256 digest.
- Removed successful placeholder output from cryptographic, analysis, image, Bombe, and Colossus compatibility operations.
- Invalid map coordinates and unsupported cryptographic parameters now fail explicitly instead of producing misleading output.
