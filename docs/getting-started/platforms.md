# Platform support

rxchef supports Linux, macOS, and Windows. The operation engine, recipe
model, JSON protocol, Store formats, and public Rust APIs are platform-neutral.
The differences that remain are installation paths, shell syntax, dynamic
library naming, terminal capabilities, and optional native dependencies.

## Platform pipeline

The repository provides three complementary checks:

| Gate | Command | What it proves |
|---|---|---|
| Linux x86_64 release | `scripts/release-check-linux.sh` | Builds and executes the complete release suite in Debian 12. |
| Windows x86_64 cross-build | `scripts/check-windows-cross.sh` | Builds CLI and TUI `.exe` artifacts with MinGW in a Debian container. |
| Native host | `scripts/check-native-platform.sh` | Runs checks, tests, Clippy, generated-doc gates, release builds, CLI discovery, and server smoke tests on the current OS. |

`scripts/check-platforms.sh` composes the container gates and automatically adds
the native gate when invoked from macOS or Windows. Apple SDKs and the Darwin
linker are supplied by Xcode and cannot be redistributed in a normal Linux
container, so macOS execution intentionally uses the native script.

The same split is encoded in `.github/workflows/platform-checks.yml`:

- an Ubuntu runner executes the complete Debian Linux release container;
- a second Ubuntu runner executes the reproducible MinGW Windows cross-build
  container;
- native macOS and Windows runners execute the common host gate.

Together these jobs enforce the supported Linux, macOS, and Windows build and
behavior contract. A release owner can additionally smoke-test the exact
archive intended for publication on its destination system.

## Linux

The Linux release container uses Debian Bookworm, stable Rust, the C/C++ build
toolchain, MkDocs, and the native libraries required by the complete feature
set. Run:

```console
./scripts/release-check-linux.sh
```

The wrapper mounts the checkout read/write, while Cargo registry and build
caches live in named Docker volumes. The pipeline does not write reports into
the repository; success or failure is communicated by its process status and
terminal output.

## macOS

Install Xcode Command Line Tools and stable Rust, then run:

```console
xcode-select --install
rustup toolchain install stable
./scripts/check-native-platform.sh
```

Homebrew can provide optional native dependencies such as Tesseract. The core,
CLI, TUI, Store, and default operation set do not require those optional
backends.

Dynamic C-library consumers link against `librxchef.dylib`. During development,
set `DYLD_LIBRARY_PATH=target/release` when the loader cannot find the library.

## Windows

Install the stable MSVC Rust toolchain and Build Tools for Visual Studio for the
normal native build:

```powershell
rustup toolchain install stable-x86_64-pc-windows-msvc
cargo build --release -p rxchef-cli
.\target\release\rxchef.exe --version
```

Git Bash, MSYS2, and PowerShell are all suitable for command execution. Replace
Unix pipelines such as `printf 'hello'` with PowerShell input where appropriate:

```powershell
'hello' | .\target\release\rxchef.exe run 'To Base64'
```

The cross-build container produces GNU `.exe` artifacts and catches target
configuration errors without requiring a Windows machine:

```console
./scripts/check-windows-cross.sh
```

## Portable behavior

The following contracts are identical on every supported platform:

- operation names and argument schemas;
- recipe version 1 and JSON/YAML serialization;
- exact output bytes and the JSON Base64 envelope;
- JSONL/JSON-RPC server framing and error codes;
- variable expansion and secret-redaction rules;
- project discovery by walking ancestor directories;
- stable CLI exit-code classes.

Paths stored in shared recipe or project files should use relative paths when
possible. Shell quoting is not part of the recipe format: once parsed, operation
names and arguments are ordinary UTF-8 strings on every platform.

## Optional native backends

The `tesseract`, `disassembly`, and `yara` features may require platform-specific
libraries or toolchains. Keep them disabled when a portable minimal binary is
the priority, or consult [Feature matrix](../reference/feature-matrix.md) before
enabling `full`.
