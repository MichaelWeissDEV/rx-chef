# `bake`: stateless recipe execution

`bake` executes an explicit JSON/YAML recipe through the shared engine without
loading or modifying saved pipeline state.

## Syntax

Exactly one recipe source is required:

```text
rxchef bake --recipe PATH [INPUT] [OUTPUT]
rxchef bake --recipe-json JSON [INPUT] [OUTPUT]
```

Input is selected with `--input`, `--input-file`, or redirected stdin. Output
uses the shared `--format` and `--output-file` contract.

## File execution

```console
rxchef bake --recipe decode.yaml \
  --input-file message.txt \
  --output-file payload.bin
```

File format is selected from `.json`, `.yaml`, or `.yml`. The document may be a
versioned recipe object or a supported bare step array.

## Inline execution

```console
rxchef bake \
  --recipe-json '[{"op":"From Base64"},{"op":"Gunzip"}]' \
  --input-file payload.txt \
  --format raw > data.bin
```

Shell quoting applies before JSON parsing. For complex recipes, a file is easier
to review and avoids shell-escape mistakes.

## Binary JSON result

```console
rxchef bake --recipe decode.yaml --input-file message.txt --format json \
  | jq '{output_len, output_is_utf8, output_base64}'
```

Decode `output_base64` for exact bytes. Display text is authoritative only when
`output_is_utf8` is true.

## Stateless behavior

`bake` does not:

- resolve a saved recipe name;
- import the recipe into global/project storage;
- append History automatically;
- discover a project for recipe lookup;
- change variables on disk.

The CLI `bake` command uses an empty variable context. Supply concrete arguments
in its recipe, or use `recipe`/`pipeline run` when Store variables and `--set`
overrides are part of the workflow. Rust callers can provide a `VariableContext`
with a low-level `ExecutionRequest` while remaining stateless.

## Validation and failure

Recipe structure and operation arguments are validated before execution. A
failure writes a diagnostic to stderr and returns an execution exit code without
emitting a successful final result. `--output-file` retains its previous file if
execution fails before the atomic replacement.

Use `bake` for build scripts, reproducible investigations, and services that
already manage recipe persistence elsewhere.

See [Recipe execution model](../concepts/recipes.md) for versioning, Fork/Merge,
Subsection, registers, labels, jumps, limits, and error behavior.
