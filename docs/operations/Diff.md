# Diff

## Overview

Compares two inputs (separated by the specified delimiter) and highlights the differences between them.

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
| 1 | Sample delimiter | `String` | no | `\\n\\n` | — | no | Delimiter separating the two input samples |
| 2 | Show added | `Boolean` | no | `true` | — | no | Show added text (wrapped in <ins> tags) |
| 3 | Show removed | `Boolean` | no | `true` | — | no | Show removed text (wrapped in <del> tags) |
| 4 | Show subtraction | `Boolean` | no | `false` | — | no | Show unchanged text |
| 5 | Ignore whitespace | `Boolean` | no | `false` | — | no | Ignore leading/trailing whitespace when comparing |
| 6 | Ignore case | `Boolean` | no | `false` | — | no | Perform case-insensitive comparison |

## How it works

Compares two inputs (separated by the specified delimiter) and highlights the differences between them.

## Implementation

The implementation is in `src/operations/diff.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Diff"
```

For file or binary input use `rxchef run "Diff" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Diff" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/diff.rs

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
