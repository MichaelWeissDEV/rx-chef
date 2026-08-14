# Regular expression

## Overview

Define your own regular expression (regex) to search the input data with, optionally choosing from a list of pre-defined patterns.

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

## How it works

Define your own regular expression (regex) to search the input data with, optionally choosing from a list of pre-defined patterns.

## Implementation

The implementation is in `src/operations/regular_expression.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Regular expression"
```

For file or binary input use `rxchef run "Regular expression" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Regular expression" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/regular_expression.rs

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
