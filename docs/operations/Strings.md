# Strings

## Overview

Extracts all strings from the input.

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

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Encoding | `String` | no | `Single byte` | — | no | Single byte, 16-bit littleendian, 16-bit bigendian, All |
| 2 | Minimum length | `UnsignedInteger` | no | `4` | — | no | Minimum string length |
| 3 | Match | `String` | no | `All printable chars (A)` | — | no | Alphanumeric + punctuation (A), All printable chars (A), Null-terminated strings (A), Alphanumeric + punctuation (U), All printable chars (U), Null-terminated strings (U) |
| 4 | Display total | `Boolean` | no | `false` | — | no | Display total count of found strings |
| 5 | Sort | `Boolean` | no | `false` | — | no | Sort results case-insensitively |
| 6 | Unique | `Boolean` | no | `false` | — | no | Remove duplicate results |

## How it works

Extracts all strings from the input.

## Implementation

The implementation is in `src/operations/strings.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Strings"
```

For file or binary input use `rxchef run "Strings" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Strings" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/strings.rs

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
