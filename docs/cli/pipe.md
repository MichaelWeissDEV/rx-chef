# Pipe command

The `pipe` command composes multiple operations in one command line without requiring you to write a separate recipe file.

## Usage

```bash
cargo run -p rxchef_cli -- pipe "to_hex,Space" "sha2,256" --input "Hello"
```

The output of the first operation becomes the input of the next operation.

## Typical use

This is the fastest way to express a small processing chain directly in the shell.

## Related pages

- [Run](run.md)
- [CLI overview](index.md)
- [Pipelines](../concepts/pipelines.md)
