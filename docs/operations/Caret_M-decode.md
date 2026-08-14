# Caret/M-decode

## Overview

Decodes caret or M-encoded strings, i.e. ^M turns into a newline, M-^] turns into 0x9d. Sources such as `cat -v`.

Please be aware that when using `cat -v` ^_ (caret-underscore) will not be encoded, but represents a valid encoding (namely that of 0x1f).

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

Declared output type: `Bytes`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

This operation has no arguments.

## How it works

Decodes caret or M-encoded strings, i.e. ^M turns into a newline, M-^] turns into 0x9d. Sources such as `cat -v`.

Please be aware that when using `cat -v` ^_ (caret-underscore) will not be encoded, but represents a valid encoding (namely that of 0x1f).

## Implementation

The implementation is in `src/operations/caret_mdecode.rs` and declares `String` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Caret/M-decode"
```

For file or binary input use `rxchef run "Caret/M-decode" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Caret/M-decode" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/caret_mdecode.rs

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
