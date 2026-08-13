# Escape Unicode Characters

## Overview

Converts characters to their unicode-escaped notations.<br><br>Supports the prefixes:<ul><li><code>\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code></code> becomes <code>\u03C3\u03BF\u03C5</code>

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
| 1 | Prefix | `String` | no | `\\u` | — | no | The prefix to use for each escape sequence |
| 2 | Encode all chars | `Boolean` | no | `false` | — | no | If true, all characters will be escaped. If false, only non-printable ASCII characters will be escaped. |
| 3 | Padding | `Integer` | no | `4` | — | no | The number of hex digits to pad to |
| 4 | Uppercase hex | `Boolean` | no | `true` | — | no | Whether to use uppercase hex digits |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/escape_unicode_characters.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Escape Unicode Characters"
```

For file or binary input use `rxchef run "Escape Unicode Characters" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Escape Unicode Characters" to_base64
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
