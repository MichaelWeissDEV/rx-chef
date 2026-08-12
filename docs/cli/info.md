# Info command

The `info` command prints metadata about a registered operation, including its name, argument list, defaults, and expected IO behavior.

## Usage

```bash
cargo run -p rxchef_cli -- info "AES Encrypt"
```

This is especially helpful when you know the operation name but need to confirm syntax or defaults before running it in a script.

## Related pages

- [List](list.md)
- [Run](run.md)
- [CLI overview](index.md)
