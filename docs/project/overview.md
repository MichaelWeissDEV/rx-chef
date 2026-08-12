# Project overview

`rxchef` is a Rust-based tooling project for working with CyberChef-style transformations in a terminal-first workflow.

The project is built around a few core ideas:

- reusable operation definitions,
- predictable pipeline execution,
- rich CLI ergonomics,
- and a clear separation between runtime logic and user interfaces.

## Goals

The main goal is to make common data transformation tasks available from the command line in a way that is easy to automate and easy to combine with shell tools.

That includes:

- hash generation and validation,
- encoding and decoding,
- encryption and decryption,
- file analysis,
- structured parsing,
- and recursive decode workflows.

## High-level architecture

The project is split into several layers:

- core library: shared logic and operation registry,
- CLI: terminal workflow access,
- TUI: interactive workbench,
- store: persisted project and variable state,
- tests: operation and integration validation.

## Runtime model

Operations are registered centrally and exposed via a common interface. A pipeline composes several operations in sequence, while the runtime handles argument parsing, input/output conversion, and error propagation.

This design makes it possible to use the same logic from:

- a shell command,
- a YAML recipe,
- a Rust library call,
- or an interactive UI.

## Current status

rxchef is actively evolving. Some areas are mature and well documented, while others still need more examples and operation-specific pages. The documentation structure reflects this and is intentionally modular so contributions can be added in small, reviewable units.

## Related pages

- [Project structure](structure.md)
- [Licensing](licensing.md)
- [Contributing](contributing.md)
- [Architecture overview](../architecture/overview.md)
