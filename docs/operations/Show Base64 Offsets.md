# Show Base64 Offsets

## Overview

Show the possible Base64 offsets for a given string.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `HTML`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Alphabet | `String` | no | `A-Za-z0-9+/=` | — | no | The Base64 alphabet to use |
| 2 | URL Safe | `Boolean` | no | `false` | — | no | Use URL-safe Base64 alphabet |
| 3 | Format | `String` | no | `Base64` | — | no | The format of the input string |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/show_base64_offsets.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Show Base64 Offsets"
```

For file or binary input use `rxchef run "Show Base64 Offsets" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Show Base64 Offsets" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
