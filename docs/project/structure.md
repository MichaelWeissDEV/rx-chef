# Project structure

The repository is organized around a small number of clear responsibilities.

## Root level

- `Cargo.toml` — workspace definition and main package metadata
- `README.md` — project overview and developer entry point
- `LICENSE` — Apache 2.0 license
- `CHANGELOG.md` — project changes and release notes
- `CONTRIBUTING.md` — development and contribution rules
- `build.rs` — build hooks for the Rust workspace

## Core source

- `src/` — core library code
  - operation registry and runtime behavior
  - pipeline execution
  - scanner and magic logic
  - shared data models

## Crates

- `crates/cli/` — command-line interface
- `crates/tui/` — terminal UI
- `crates/store/` — persistent storage and project state
- `crates/xtask/` — project automation and helper tasks

## Tests

- `tests/` — end-to-end and operation-level tests

## Documentation

- `docs/` — MkDocs documentation source
  - `getting-started/` for onboarding content
  - `architecture/` for technical internals
  - `concepts/` for user-facing model explanations
  - `cli/` for command-line usage
  - `operations/` for operation-by-operation pages
  - `reference/` for generated and curated reference material
  - `development/` for contributor workflows

## How to navigate the codebase

If you are new to the project, a typical start is:

1. Read the README and this project overview.
2. Review the CLI docs and quickstart guide.
3. Inspect the operational pages in the docs folder.
4. Move into the core library and CLI crates when implementing changes.

## Related pages

- [Project overview](overview.md)
- [Licensing](licensing.md)
- [Contributing](contributing.md)
- [Architecture overview](../architecture/overview.md)
