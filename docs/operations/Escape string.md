# Escape string

## Overview

Escapes special characters in a string so that they do not cause conflicts. For example, <code>Don't stop me now</code> becomes <code>Don\'t stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\n</code> (Line feed/newline)</li><li><code>\r</code> (Carriage return)</li><li><code>\t</code> (Horizontal tab)</li><li><code>\b</code> (Backspace)</li><li><code>\f</code> (Form feed)</li><li><code>\xnn</code> (Hex, where n is 0-f)</li><li><code>\\</code> (Backslash)</li><li><code>\'</code> (Single quote)</li><li><code>\&quot;</code> (Double quote)</li><li><code>\unnnn</code> (Unicode character)</li><li><code>\u{nnnnnn}</code> (Unicode code point)</li></ul>

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Exact` |
| Availability | Available |
| Input requirement | `Required` |
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
| 1 | Escape level | `String` | no | `Special chars` | — | no | The level of escaping to perform |
| 2 | Escape quote | `String` | no | `Single` | — | no | Which type of quote to escape |
| 3 | JSON compatible | `Boolean` | no | `false` | — | no | Whether to ensure the output is JSON compatible |
| 4 | ES6 compatible | `Boolean` | no | `true` | — | no | Whether to use ES6 unicode escape sequences (\\u{...}) |
| 5 | Uppercase hex | `Boolean` | no | `false` | — | no | Whether to use uppercase hex digits |

## Implementation

The implementation is in `src/operations/escape_string.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Escape string" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Exact`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/escape_string.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
