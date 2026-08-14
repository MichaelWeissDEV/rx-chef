# Escape Unicode Characters

## Overview

Converts characters to their unicode-escaped notations.<br><br>Supports the prefixes:<ul><li><code>\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code></code> becomes <code>\u03C3\u03BF\u03C5</code>

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
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

Converts characters to their unicode-escaped notations.<br><br>Supports the prefixes:<ul><li><code>\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code></code> becomes <code>\u03C3\u03BF\u03C5</code>

## Implementation

The implementation is in `src/operations/escape_unicode_characters.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

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

Correctness:
- tests/tests/operations/escape_unicode_characters.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
