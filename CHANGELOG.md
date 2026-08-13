# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
