# Find / Replace

## Overview

Replaces all occurrences of the first string with the second. Supports regex, simple string, and extended string modes.

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

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Find | `String` | no | `<empty>` | — | no | The string or regex to find |
| 2 | Find type | `String` | no | `Simple string` | — | no | Regex, Extended (\\n, \\t, \\x...), or Simple string |
| 3 | Replace | `String` | no | `<empty>` | — | no | The replacement string |
| 4 | Global match | `Boolean` | no | `true` | — | no | Replace all occurrences |
| 5 | Case insensitive | `Boolean` | no | `false` | — | no | Ignore case when matching |
| 6 | Multiline matching | `Boolean` | no | `true` | — | no | ^ and $ match start/end of lines |
| 7 | Dot matches all | `Boolean` | no | `false` | — | no | Dot also matches newline |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/find_replace.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Find / Replace"
```

For file or binary input use `rxchef run "Find / Replace" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Find / Replace" to_base64
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
