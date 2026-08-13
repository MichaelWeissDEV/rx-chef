# History

The CLI keeps a lightweight history of recent runs so commands can be repeated or reviewed more easily.

## Purpose

History helps with:

- repeating workflows,
- checking recent command patterns,
- and reusing successful transformations during debugging.

## Typical use

History is useful when you are iterating on a chained workflow and want to inspect what you ran previously without rewriting the full command.

Only byte counts and bounded previews are retained, not the original payload.
Sensitive operation arguments are recorded as `<redacted>`, and stored variable
references are not expanded into History. Replaying an entry therefore requires
fresh input:

```console
rxchef history run ID --input-file payload.bin
```

A preview is never substituted for the original input.

## Related pages

- [Run](run.md)
- [Pipe](pipe.md)
- [CLI overview](index.md)
