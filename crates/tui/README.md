# rxchef-tui

The `rxchef_tui` interactive terminal interface for
[rx-chef](https://rx-chef.readthedocs.io/), a CyberChef-style toolkit of
cryptographic, encoding, and data transformation operations, ported to Rust.

```bash
cargo install rxchef-tui
rxchef_tui
```

See the [TUI documentation](https://rx-chef.readthedocs.io/en/latest/tui/overview/)
for usage and keybindings, and the
[project overview](https://rx-chef.readthedocs.io/) for the library, CLI,
and server interfaces.

License: Apache-2.0

Recipes run explicitly with `r`. Editing, loading, or restoring a recipe and
changing input does not execute operations automatically. Trace views show
actual completed actions, including repeated branch steps, with bounded binary
prefixes and explicit truncation. Final output is shown separately. Execution
uses `rxchef-client`; a remote TUI connection mode is not implemented yet.
