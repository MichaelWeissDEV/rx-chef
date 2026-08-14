# ROT13

## Overview

Rotates each letter by 13 positions in the alphabet. ROT13 is a simple Caesar cipher.

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
| 1 | Rotate lower case chars | `Boolean` | no | `true` | — | no | Apply rotation to a-z |
| 2 | Rotate upper case chars | `Boolean` | no | `true` | — | no | Apply rotation to A-Z |
| 3 | Rotate digits | `Boolean` | no | `false` | — | no | Apply ROT5 to digits 0-9 |
| 4 | Amount | `Integer` | no | `13` | — | no | Amount to rotate (default 13) |

## How it works

Rotates each letter by 13 positions in the alphabet. ROT13 is a simple Caesar cipher.

## Implementation

The implementation is in `src/operations/rot13.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "ROT13"
```

For file or binary input use `rxchef run "ROT13" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "ROT13" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/rot13.rs

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
