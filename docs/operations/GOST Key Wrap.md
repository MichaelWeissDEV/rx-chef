# GOST Key Wrap

## Overview

A key wrapping algorithm for protecting keys in untrusted storage using one of the GOST block ciphers.

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

Declared output type: `Bytes`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | The Key Encryption Key (KEK). |
| 2 | User Key Material | `String` | no | `<empty>` | — | no | User Key Material (UKM). |
| 3 | Input type | `Enum` | no | `Raw` | Raw, Hex | no | Input encoding (Raw, Hex) |
| 4 | Output type | `Enum` | no | `Hex` | Hex, Raw | no | Output encoding (Hex, Raw) |
| 5 | Algorithm | `String` | no | `GOST 28147 (1989)` | — | no | The GOST algorithm to use. |
| 6 | sBox | `String` | no | `E-TEST` | — | no | The sBox to use (only for GOST 28147 (1989)). |
| 7 | Key wrapping | `String` | no | `NO` | — | no | The key wrapping mode. |

## How it works

A key wrapping algorithm for protecting keys in untrusted storage using one of the GOST block ciphers.

## Implementation

The implementation is in `src/operations/gost_key_wrap.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "GOST Key Wrap"
```

For file or binary input use `rxchef run "GOST Key Wrap" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "GOST Key Wrap" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/gost_key_wrap.rs

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
