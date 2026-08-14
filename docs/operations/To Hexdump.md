# To Hexdump

## Overview

Creates a hexdump of the input data, displaying both the hexadecimal values of each byte and an ASCII representation alongside.

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
| 1 | Width | `UnsignedInteger` | no | `16` | — | no | Number of bytes per row (must be >= 1) |
| 2 | Upper case hex | `Boolean` | no | `false` | — | no | Display hex bytes in upper case |
| 3 | Include final length | `Boolean` | no | `false` | — | no | Append the total byte count as a final line |
| 4 | UNIX format | `Boolean` | no | `false` | — | no | Use UNIX printable character subset for ASCII column |

## How it works

Creates a hexdump of the input data, displaying both the hexadecimal values of each byte and an ASCII representation alongside.

## Implementation

The implementation is in `src/operations/to_hexdump.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "To Hexdump"
```

For file or binary input use `rxchef run "To Hexdump" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "To Hexdump" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/to_hexdump.rs

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
