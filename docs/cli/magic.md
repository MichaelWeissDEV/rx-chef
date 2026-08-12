# Magic command

The `magic` command tries to identify and decode layered encodings automatically.

## Purpose

This is useful when the input looks encoded, but the exact sequence of transformations is not obvious. Instead of manually debugging the chain, the tool explores likely decode paths and ranks the most plausible results.

## Example

```bash
rxchef magic --input "U0dWc2JHOD0="
```

This can unwrap nested layers such as double-encoded text or other common transformations.

## Related pages

- [Scan](scan.md)
- [Run](run.md)
- [CLI overview](index.md)
