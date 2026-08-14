# Regular expression

## Overview

Define your own regular expression (regex) to search the input data with, optionally choosing from a list of pre-defined patterns.

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
| 1 | Built in regexes | `Regex` | no | `User defined` | — | no | Built in regexes |
| 2 | Regex | `Regex` | no | `<empty>` | — | no | Regular expression |
| 3 | Case insensitive | `Boolean` | no | `true` | — | no | Case insensitive |
| 4 | ^ and $ match at newlines | `Boolean` | no | `true` | — | no | ^ and $ match at newlines |
| 5 | Dot matches all | `Boolean` | no | `false` | — | no | Dot matches all |
| 6 | Unicode support | `Boolean` | no | `false` | — | no | Unicode support |
| 7 | Astral support | `Boolean` | no | `false` | — | no | Astral support |
| 8 | Display total | `Boolean` | no | `false` | — | no | Display total |
| 9 | Output format | `String` | no | `Highlight matches` | — | no | Output format |

## Implementation

The implementation is in `src/operations/regular_expression.rs` and declares `String` input and `HTML` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Regular expression" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/regular_expression.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
