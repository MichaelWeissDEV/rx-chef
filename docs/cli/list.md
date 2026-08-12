# List command

The `list` command shows operations that are currently available in the registry.

## Usage

```bash
cargo run -p rxchef_cli -- list
```

You can also filter by keyword:

```bash
cargo run -p rxchef_cli -- list hash
cargo run -p rxchef_cli -- list base64
```

## Use cases

- discover available operations,
- find the correct operation name,
- narrow the catalog by functionality,
- and verify that a command is present before scripting it.

## Related pages

- [Info](info.md)
- [Run](run.md)
- [CLI overview](index.md)
