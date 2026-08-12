# Installation

This page explains how to install and build rxchef for local development or direct use from the command line.

## Requirements

- Rust 1.75 or newer
- A C compiler for native dependencies such as `capstone` and `yara-x`
- Git for cloning the repository

## Clone the repository

```bash
git clone https://github.com/MichaelWeissDEV/rx-chef.git
cd rx-chef
```

## Build the project

```bash
cargo build --release
```

This produces release binaries in the `target/release` directory, including the main CLI and the TUI.

## Run the CLI

```bash
cargo run -p rxchef_cli -- --help
```

or directly after building:

```bash
./target/release/rxchef --help
```

## Optional: install locally

If you want to install the binary into your Cargo bin directory:

```bash
cargo install --path .
```

## Development notes

The workspace contains multiple crates, including the core library, the CLI, the TUI, and the store layer. For most contributors, the main entry points are:

- `src/` for the core library and runtime logic
- `crates/cli/` for the command-line interface
- `crates/tui/` for the terminal UI
- `tests/` for integration and operation-level tests

## Troubleshooting

If compilation fails, verify that:

- your Rust toolchain is up to date,
- the required system build tools are installed,
- you are working from the repository root and not from a nested crate.

For the full development workflow, see [development/building.md](../development/building.md).
