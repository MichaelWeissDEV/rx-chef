# Escape string

## Overview

Escapes special characters in a string so that they do not cause conflicts. For example, <code>Don't stop me now</code> becomes <code>Don\'t stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\n</code> (Line feed/newline)</li><li><code>\r</code> (Carriage return)</li><li><code>\t</code> (Horizontal tab)</li><li><code>\b</code> (Backspace)</li><li><code>\f</code> (Form feed)</li><li><code>\xnn</code> (Hex, where n is 0-f)</li><li><code>\\</code> (Backslash)</li><li><code>\'</code> (Single quote)</li><li><code>\&quot;</code> (Double quote)</li><li><code>\unnnn</code> (Unicode character)</li><li><code>\u{nnnnnn}</code> (Unicode code point)</li></ul>

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
| 1 | Escape level | `String` | no | `Special chars` | — | no | The level of escaping to perform |
| 2 | Escape quote | `String` | no | `Single` | — | no | Which type of quote to escape |
| 3 | JSON compatible | `Boolean` | no | `false` | — | no | Whether to ensure the output is JSON compatible |
| 4 | ES6 compatible | `Boolean` | no | `true` | — | no | Whether to use ES6 unicode escape sequences (\\u{...}) |
| 5 | Uppercase hex | `Boolean` | no | `false` | — | no | Whether to use uppercase hex digits |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/escape_string.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Escape string"
```

For file or binary input use `rxchef run "Escape string" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Escape string" to_base64
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
