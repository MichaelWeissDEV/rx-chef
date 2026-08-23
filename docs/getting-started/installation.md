# Installation

rx-chef ships as prebuilt binaries, native OS packages, and Rust crates.
Pick whichever fits your platform; all of them install the same `rxchef`
CLI and `rxchef_tui` terminal interface.

!!! note
    The package-manager and prebuilt-binary paths below become available
    starting with the first tagged release. Until then, use
    [Build from source](#build-from-source).

## Cargo

```bash
cargo install rxchef-cli
cargo install rxchef-tui   # optional, the interactive terminal interface
```

Add the library to a Rust project:

```bash
cargo add rxchef
```

## Homebrew (macOS and Linux)

```bash
brew install MichaelWeissDEV/tap/rxchef
brew install MichaelWeissDEV/tap/rxchef-tui   # optional
```

This is a dedicated tap (`MichaelWeissDEV/homebrew-tap`), not
`homebrew/core`.

## Arch Linux / AUR

```bash
yay -S rxchef
# or: paru -S rxchef
```

This builds from the tagged release source, not a binary — pacman/AUR
helpers compile it locally. The build takes a few minutes.

## Debian / Ubuntu

Download `rxchef_<version>_<arch>.deb` and `rxchef-tui_<version>_<arch>.deb`
from the [GitHub Releases page](https://github.com/MichaelWeissDEV/rx-chef/releases),
then:

```bash
sudo apt install ./rxchef_<version>_<arch>.deb
sudo apt install ./rxchef-tui_<version>_<arch>.deb   # optional
```

`<arch>` is `amd64` or `arm64`.

## Fedora / RHEL / openSUSE

Download `rxchef-<version>-1.<arch>.rpm` and
`rxchef-tui-<version>-1.<arch>.rpm` from the
[GitHub Releases page](https://github.com/MichaelWeissDEV/rx-chef/releases),
then:

```bash
sudo dnf install ./rxchef-<version>-1.<arch>.rpm          # Fedora/RHEL
sudo zypper install ./rxchef-<version>-1.<arch>.rpm       # openSUSE
```

`<arch>` is `x86_64` or `aarch64`.

## Prebuilt binaries (any platform)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/MichaelWeissDEV/rx-chef/releases/latest/download/rxchef-cli-installer.sh | sh
```

Or download the archive for your platform directly from the
[GitHub Releases page](https://github.com/MichaelWeissDEV/rx-chef/releases):
Linux (x86_64/aarch64), macOS (Intel/Apple Silicon), or Windows (x86_64).
Each release also publishes `SHA256SUMS` for verification.

## Build from source

Requirements:

- Rust 1.96 or newer
- A C compiler for native dependencies (only needed with optional features
  such as `disassembly`/`yara`)
- Git

```bash
git clone https://github.com/MichaelWeissDEV/rx-chef.git
cd rx-chef
cargo build --release
```

This produces `target/release/rxchef` (CLI) and `target/release/rxchef_tui`
(TUI). Run directly, or install into your Cargo bin directory:

```bash
cargo install --path crates/cli
cargo install --path crates/tui   # optional
```

Generate shell completions or the manual page from the installed binary:

```console
rxchef completions bash > rxchef.bash
rxchef completions zsh > _rxchef
rxchef manpage --output rxchef.1
```

The workspace contains multiple crates: `src/` (core library), `crates/cli/`
(CLI), `crates/tui/` (TUI), `crates/store/` (recipes/variables/history), and
`tests/` (integration tests). See [development/building.md](../development/building.md)
for the full contributor workflow, and
[reference/feature-matrix.md](../reference/feature-matrix.md) for optional
Cargo features (OCR, PGP, YARA, disassembly) and their native dependencies.

## Troubleshooting

If compilation fails, verify that:

- your Rust toolchain is up to date,
- the required system build tools are installed for any optional features
  you enabled,
- you are working from the repository root and not from a nested crate.

## Project resources

- [Published documentation](https://rx-chef.readthedocs.io/en/latest/)
- [GitHub repository](https://github.com/MichaelWeissDEV/rx-chef)
- [Issue tracker](https://github.com/MichaelWeissDEV/rx-chef/issues)
