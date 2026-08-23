# Building the project

The rxchef workspace is built with Cargo.

## Standard build

```bash
cargo build
```

## Release build

```bash
cargo build --release
```

## Workspace tests

```bash
cargo test --workspace
```

## Useful project notes

- the core library lives in `src/`,
- CLI code is in `crates/cli/`,
- the TUI is in `crates/tui/`,
- and supporting state logic lives in `crates/store/`.

## Related pages

- [Testing](testing.md)
- [Releasing](releasing.md)
- [Architecture overview](../architecture/overview.md)
