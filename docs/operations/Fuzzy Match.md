# Fuzzy Match

## Overview

Conducts a fuzzy search to find a pattern within the input based on weighted criteria.

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
| 1 | Search | `String` | no | `<empty>` | — | no | Pattern to search for |
| 2 | Sequential bonus | `Integer` | no | `15` | — | no | Bonus for adjacent matches |
| 3 | Separator bonus | `Integer` | no | `30` | — | no | Bonus if match occurs after a separator |
| 4 | Camel bonus | `Integer` | no | `30` | — | no | Bonus if match is uppercase and previous is lower |
| 5 | First letter bonus | `Integer` | no | `15` | — | no | Bonus if the first letter is matched |
| 6 | Leading letter penalty | `Integer` | no | `-5` | — | no | Penalty applied for every letter in the input before the first match |
| 7 | Max leading letter penalty | `Integer` | no | `-15` | — | no | Maxiumum penalty for leading letters |
| 8 | Unmatched letter penalty | `Integer` | no | `-1` | — | no | Unmatched letter penalty |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/fuzzy_match.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Fuzzy Match"
```

For file or binary input use `rxchef run "Fuzzy Match" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Fuzzy Match" to_base64
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
