# Operation matrix

The generated [operation index](../operations/index.md) is the authoritative
matrix for this release. It contains every runtime-registered operation grouped
by module, with a dedicated page for input/output types, arguments, defaults,
and the `broken` marker.

For machine consumers, do not parse Markdown:

```console
rxchef operations --json
rxchef operation describe 'From Base64' --json
```

`broken: true` means the operation is discoverable for compatibility but cannot
produce a trustworthy result in this build. Optional feature availability is
reported by the descriptor and documented in the [feature matrix](feature-matrix.md).
