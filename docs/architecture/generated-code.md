# Generated code

Some parts of rxchef rely on generated artifacts so the operation catalog and related metadata can be extended without manual duplication across multiple layers.

The operation registry is generated explicitly:

```console
cargo xtask generate-registry
cargo xtask check-registry
```

`check-registry` renders and formats a copy under `target/xtask` before comparing
it byte-for-byte with `src/operations/mod.rs`. Normal Cargo builds never write
to committed source files.

## Why generation exists

The project contains many operations with similar metadata requirements:

- names,
- argument definitions,
- default values,
- input and output types,
- and CLI metadata.

Maintaining these definitions manually in multiple places would be error-prone and harder to keep consistent. Generating shared metadata from a canonical source reduces drift.

## Typical generated artifacts

Generated output may include:

- operation reference pages,
- operation matrices,
- metadata indexes,
- or CLI-facing documentation snippets.

This is especially helpful when the operation registry changes often while the docs need to stay in sync with the actual code.

## Benefits

- fewer inconsistencies between runtime and docs,
- safer additions of new operations,
- easier regeneration when the registry changes,
- better reproducibility for release and documentation workflows.

## Related pages

- [Registry](registry.md)
- [Operation model](operation-model.md)
- [Architecture overview](overview.md)
