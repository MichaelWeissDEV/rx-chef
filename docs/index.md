# rxchef documentation

rxchef brings CyberChef-style data transformations to the terminal, Rust
applications, and editor integrations. A single execution engine and operation
registry power the command-line interface, reusable library, interactive TUI,
C-compatible FFI, and persistent JSONL/JSON-RPC server.

[Get started](getting-started/installation.md){ .md-button }
[Browse 478 operations](operations/index.md){ .md-button }
[View the source on GitHub](https://github.com/MichaelWeissDEV/rx-chef){ .md-button }

## What rxchef provides

- **478 registered operations** for encoding, cryptography, hashing,
  compression, structured data, networking, forensics, and image processing.
- **Unix-native command behavior** with stdin/stdout pipelines, files, exact
  binary input, structured JSON output, and useful contextual errors.
- **Reproducible JSON/YAML recipes** plus saved pipelines, variables, projects,
  history, forks, subsections, registers, labels, and bounded jumps.
- **Reusable Rust APIs** for discovery, direct execution, typed pipelines,
  complete recipes, Magic, Scan, and the stdio protocol.
- **Persistent editor integration** designed for Neovim and other local plugin
  clients without a frontend-specific backend.
- **Generated operation documentation** kept synchronized with runtime metadata
  and checked by the local Linux release pipeline.

rxchef targets Linux, macOS, and Windows. Most of the code is portable Rust;
optional native backends and terminal behavior are described explicitly in the
installation and feature documentation. A reproducible Linux x86_64 container
executes the complete local release pipeline, while native hosts can run the
same Cargo, documentation, protocol, and smoke-test gates directly.

## Start here

| Goal | Documentation |
|---|---|
| Install or build rxchef | [Installation](getting-started/installation.md) |
| Build on Linux, macOS, or Windows | [Platform support](getting-started/platforms.md) |
| Run the first transformations | [Quickstart](getting-started/quickstart.md) |
| Learn the complete terminal workflow | [CLI handbook](cli/handbook.md) |
| Understand stdin, files, and stdout | [Input and output](concepts/input-output.md) |
| Explore every CLI command | [CLI reference](cli/reference.md) |
| Compose operations | [Pipelines](concepts/pipelines.md) and [recipes](cli/recipes.md) |
| Integrate an editor or plugin | [Editor integration protocol](cli/integration.md) |
| Embed rxchef in Rust | [Rust library](library.md) |
| Find an operation and its arguments | [Operation catalog](operations/index.md) |
| Understand optional backends | [Feature matrix](reference/feature-matrix.md) |
| Understand internal component flow | [System design](architecture/system-design.md) |
| Contribute to the project | [Contributing](project/contributing.md) |

## Quick example

```console
# Decode Base64 using literal input
rxchef run "From Base64" --input "SGVsbG8="

# Compose transformations using stdin/stdout
printf 'hello' | rxchef pipe "To Upper Case" "To Base64"

# Inspect the machine-readable operation registry
rxchef operations --json

# Start a persistent process for an editor plugin
rxchef serve --stdio
```

Operation names are normalized, so human-readable names and shell-friendly
aliases such as `To Base64`, `to_base64`, and `tobase64` resolve through the
same registry.

## Project links

- [GitHub repository](https://github.com/MichaelWeissDEV/rx-chef)
- [Issue tracker](https://github.com/MichaelWeissDEV/rx-chef/issues)
- [Pull requests](https://github.com/MichaelWeissDEV/rx-chef/pulls)
- [Apache 2.0 license](https://github.com/MichaelWeissDEV/rx-chef/blob/master/LICENSE)
- [CyberChef upstream project](https://github.com/gchq/CyberChef)

Every documentation page includes a repository link in the header and an edit
link for proposing changes to its source file.
