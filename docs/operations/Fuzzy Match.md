# Fuzzy Match

## Overview

Conducts a fuzzy search to find a pattern within the input based on weighted criteria.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Required` |
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

## Implementation

The implementation is in `src/operations/fuzzy_match.rs` and declares `String` input and `HTML` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Fuzzy Match" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `HTML` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/fuzzy_match.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
