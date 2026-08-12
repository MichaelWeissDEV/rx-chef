# CLI overview

The rxchef CLI is the primary interface for executing operations, composing pipelines, and managing saved state.

## Typical usage

```bash
rxchef list hash
rxchef info "From Base64"
rxchef run "From Base64" --input "SGVsbG8="
rxchef pipe to_upper_case to_base64 --input "hello"
```

## Common command groups

- `list` discovers available operations,
- `info` shows the metadata and parameters for an operation,
- `run` executes one operation,
- `pipe` composes several operations inline,
- `scan` searches files or streams for suspicious or encoded content,
- `magic` recursively attempts decode chains,
- and `var` / project commands manage persistent state.

## Input and output model

CLI commands accept:

- direct input via `--input`,
- file input via `--input-file`,
- or piped stdin when no explicit input is supplied.

Output is streamed to stdout while diagnostics and trace information remain on stderr, so Unix pipelines stay clean and composable.

## Related pages

- [List](list.md)
- [Info](info.md)
- [Run](run.md)
- [Pipe](pipe.md)
- [Scan](scan.md)
- [Magic](magic.md)
