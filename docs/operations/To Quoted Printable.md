# To Quoted Printable

## Overview

Quoted-Printable, or QP encoding, is an encoding using printable ASCII characters (alphanumeric and the equals sign '=') to transmit 8-bit data over a 7-bit data path or, generally, over a medium which is not 8-bit clean. It is defined as a MIME content transfer encoding for use in email.<br><br>QP works by using the equals sign '=' as an escape character. It also limits line length to 76, as some software has limits on line length.

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

This operation has no arguments.

## How it works

Quoted-Printable, or QP encoding, is an encoding using printable ASCII characters (alphanumeric and the equals sign '=') to transmit 8-bit data over a 7-bit data path or, generally, over a medium which is not 8-bit clean. It is defined as a MIME content transfer encoding for use in email.<br><br>QP works by using the equals sign '=' as an escape character. It also limits line length to 76, as some software has limits on line length.

## Implementation

The implementation is in `src/operations/to_quoted_printable.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "To Quoted Printable"
```

For file or binary input use `rxchef run "To Quoted Printable" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "To Quoted Printable" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/to_quoted_printable.rs

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
