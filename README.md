# rxchef

[![Documentation](https://readthedocs.org/projects/rx-chef/badge/?version=latest)](https://rx-chef.readthedocs.io/en/latest/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/MichaelWeissDEV/rx-chef/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg?logo=rust)](https://www.rust-lang.org/)

CyberChef-style data transformations for the terminal, Rust applications, and
editor integrations.

rxchef provides one shared execution engine and operation registry across a
native command-line interface, reusable Rust library, interactive TUI,
experimental C-compatible FFI, and persistent JSONL/JSON-RPC server. Its 478 registered operations cover
encoding, cryptography, hashing, compression, structured data, networking,
forensics, image processing, and more.

**[Documentation](https://rx-chef.readthedocs.io/en/latest/)** ·
**[CLI reference](https://rx-chef.readthedocs.io/en/latest/cli/reference/)** ·
**[CLI handbook](https://rx-chef.readthedocs.io/en/latest/cli/handbook/)** ·
**[Operation catalog](https://rx-chef.readthedocs.io/en/latest/operations/)** ·
**[Rust library](https://rx-chef.readthedocs.io/en/latest/library/)** ·
**[GitHub repository](https://github.com/MichaelWeissDEV/rx-chef)**

## Highlights

- **478 discoverable operations** with generated metadata and documentation.
- **Unix-native CLI** with clean stdin/stdout behavior, binary input, files,
  inline pipelines, JSON/YAML recipes, variables, projects, and run history.
- **Composable recipe engine** supporting `Fork`/`Merge`, `Subsection`,
  registers, labels, and bounded conditional or unconditional jumps.
- **Machine-readable integration API** for operation discovery, descriptions,
  direct execution, and complete recipes.
- **Persistent stdio server** designed for Neovim, editor plugins, and other
  local clients using JSONL or JSON-RPC 2.0.
- **Reusable Rust library** without terminal assumptions, plus an explicitly
  experimental C ABI.
- **Magic and Scan workflows** for recursive decoding and streaming discovery
  across files, directories, memory dumps, captures, or piped input.
- **Generated Read the Docs site** whose 478 operation pages are checked for
  freshness by the local release check.

## Release verification

rxchef is written as a portable Rust application for Linux, macOS, and Windows.
The repository includes a reproducible Linux x86_64 container gate at
`./scripts/release-check-linux.sh`, a Windows GNU cross-build container, and
native macOS and Windows jobs. The same gates are wired into the GitHub Actions
platform matrix; they are also directly runnable before a local release.

Platform setup, the Linux release container, Windows cross-build container, and
native host gates are documented in the
[platform guide](https://rx-chef.readthedocs.io/en/latest/getting-started/platforms/).

## Installation

```bash
cargo install rxchef-cli                          # Cargo
brew install MichaelWeissDEV/tap/rxchef           # Homebrew (macOS/Linux)
sudo apt install ./rxchef_<version>_<arch>.deb    # Debian/Ubuntu
sudo dnf install ./rxchef-<version>-1.<arch>.rpm  # Fedora/RHEL/openSUSE
```

`.deb`/`.rpm` packages, an AUR `PKGBUILD`, and platform archives are
attached to each
[GitHub Release](https://github.com/MichaelWeissDEV/rx-chef/releases)
(the AUR package isn't published to `aur.archlinux.org` yet — see the
[installation guide](https://rx-chef.readthedocs.io/en/latest/getting-started/installation/)).
Building from source only needs the stable Rust toolchain and Git:

```console
git clone https://github.com/MichaelWeissDEV/rx-chef.git
cd rx-chef
cargo install --path crates/cli
rxchef --version
```

Optional OpenPGP, JSON query, OCR, Capstone disassembly, and YARA support is
controlled through Cargo features (`pgp`, `jsonata`, `tesseract`,
`disassembly`, and `yara`; `full` enables all of them). OCR additionally
requires a system Tesseract/Leptonica installation.
See the [installation guide](https://rx-chef.readthedocs.io/en/latest/getting-started/installation/)
and [feature matrix](https://rx-chef.readthedocs.io/en/latest/reference/feature-matrix/)
for platform-specific details.

## Quick start

Discover operations and inspect their argument schemas:

```console
rxchef list base64
rxchef info "From Base64"
rxchef operations --json
rxchef operation describe "From Base64" --json
```

Run a single operation using literal input, a file, or stdin:

```console
rxchef run "From Base64" --input "SGVsbG8="
rxchef run "Detect File Type" --input-file sample.bin
printf 'hello' | rxchef run "To Upper Case"
```

Chain any number of operations in a pipe-clean workflow:

```console
printf 'hello' | rxchef pipe "to_upper_case" "to_base64"
rxchef pipe "to_hex,Space" "sha2,256" --input "Hello"
```

Operation names are normalized, so `to_hex`, `ToHex`, and `"To Hex"` resolve
to the same registry entry. Typed arguments use prefixes such as `num:12.5`,
`bool:true`, and `hex:48656c6c6f`.

For the full command surface and shell-composition rules, read the
[CLI handbook](https://rx-chef.readthedocs.io/en/latest/cli/handbook/) and
[complete command reference](https://rx-chef.readthedocs.io/en/latest/cli/reference/).

## Recipes and flow control

Reproducible recipes can be supplied as JSON or YAML files, or directly as an
inline JSON array:

```console
rxchef bake \
  --recipe-json '[{"op":"To Upper Case"},{"op":"To Base64"}]' \
  --input "Hello"

printf 'one\ntwo' | rxchef pipe 'Fork,\n,|,false' 'To Upper Case' Merge
```

The same flow-aware recipe engine backs `bake`, inline pipes, saved recipes,
projects, the Rust integration API, and plugin requests. See
[recipes and flow control](https://rx-chef.readthedocs.io/en/latest/cli/recipes/)
for nested forks, subsections, registers, labels, jumps, error handling, and
binary-safe behavior.

## Editor and plugin integration

Start one long-lived process per client session:

```console
rxchef serve --stdio
```

Write one JSON request per line to stdin and read one compact JSON response per
line from stdout:

```json
{"id":1,"method":"operations"}
{"id":2,"method":"describe","params":{"operation":"XOR"}}
{"id":3,"method":"bake","params":{"input":"Hello","recipe":[{"op":"To Base64"}]}}
{"id":4,"method":"shutdown"}
```

Exact binary values use Base64 result envelopes. The server supports compact
JSONL requests and JSON-RPC 2.0, notifications, structured errors, and clean
shutdown. The complete contract is documented in the
[editor integration protocol](https://rx-chef.readthedocs.io/en/latest/cli/integration/).

## Rust library

The core crate can be embedded independently of the CLI:

```rust
use rxchef::{catalog, execute};

let descriptor = catalog::describe("to_base64")?;
let result = execute::run("To Base64", b"Hello".to_vec(), vec![])?;

assert_eq!(descriptor.name, "To Base64");
assert_eq!(result.output, b"SGVsbG8=");
# Ok::<(), Box<dyn std::error::Error>>(())
```

Lower-level APIs expose typed operation values, direct registry lookup,
pipelines, Magic, scanning, and `serve_jsonl(reader, writer)`. See the
[Rust library guide](https://rx-chef.readthedocs.io/en/latest/library/).

## Magic and streaming scan

Recursively identify and unwrap layered encodings:

```console
rxchef magic --input "U0dWc2JHOD0=" --decode
rxchef magic --input "…" --crib "flag{" --intensive --depth 5
```

Search large files, directories, or stdin without loading the entire input into
memory:

```console
rxchef scan dump.bin --decode
rxchef scan ./captures --recursive --json | jq 'select(.kinds[] == "From Hex")'
cat memory.dump | rxchef scan --entropy 4.5
```

Machine output stays on stdout; diagnostics and counts use stderr. More details
are available in the [Magic](https://rx-chef.readthedocs.io/en/latest/cli/magic/)
and [Scan](https://rx-chef.readthedocs.io/en/latest/cli/scan/) guides.

## Documentation

The complete documentation is published at
**[rx-chef.readthedocs.io](https://rx-chef.readthedocs.io/en/latest/)** and
includes:

- installation and guided quick starts;
- complete CLI behavior, input precedence, output formats, and exit codes;
- recipes, saved pipelines, variables, projects, and persistence;
- all 478 generated operation pages and their ordered argument schemas;
- the Rust library, FFI, architecture, editor protocol, and security model;
- testing, benchmarking, fuzzing, and release procedures.

Build the exact Read the Docs site locally with:

```console
python -m pip install -r docs/requirements.txt
mkdocs build --strict
```

## Development and verification

```console
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo test --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets -- \
  -D clippy::correctness -D clippy::suspicious
cargo run -p xtask -- docs --check
cargo xtask check-registry
cargo xtask audit-operations
cargo run --example generate_operation_docs -- --check
mkdocs build --strict
```

The current v0.1.0 workspace registers 478 operations. Quality evidence and
remaining unknown parity are published in the generated operation matrix; the
repository does not equate registration with complete verification.

## Contributing

Bug reports, focused pull requests, authoritative test vectors, documentation
improvements, and integration examples are welcome.

- [Open an issue](https://github.com/MichaelWeissDEV/rx-chef/issues)
- [View pull requests](https://github.com/MichaelWeissDEV/rx-chef/pulls)
- [Read the contribution guide](https://rx-chef.readthedocs.io/en/latest/project/contributing/)
- [Review the project structure](https://rx-chef.readthedocs.io/en/latest/project/structure/)

## License and attribution

rxchef is licensed under the
[Apache License 2.0](https://github.com/MichaelWeissDEV/rx-chef/blob/master/LICENSE).
It ports and adapts operation behavior from
[CyberChef](https://github.com/gchq/CyberChef), originally developed by GCHQ.
