# Run command

The `run` command executes a single operation on the supplied input.

## Usage

```bash
cargo run -p rxchef_cli -- run "From Base64" --input "SGVsbG8="
```

You can also pass positional arguments:

```bash
cargo run -p rxchef_cli -- run "SHA2" --input "hello" "256"
```

## Typical use cases

- quick transformations,
- debugging a single operation,
- scripting a one-off conversion,
- and validating an operation without a full pipeline.

## Related pages

- [Pipe](pipe.md)
- [List](list.md)
- [Info](info.md)
