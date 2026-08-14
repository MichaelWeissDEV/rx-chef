# Parse TLV

## Overview

Converts a Type-Length-Value (TLV) encoded string into a JSON object. Can optionally include a Key / Type entry.

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

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Type/Key size | `UnsignedInteger` | no | `1` | — | no | Size of the type/key field in bytes |
| 2 | Length size | `UnsignedInteger` | no | `1` | — | no | Size of the length field in bytes |
| 3 | Use BER | `Boolean` | no | `false` | — | no | Use Basic Encoding Rules for length field |

## How it works

Converts a Type-Length-Value (TLV) encoded string into a JSON object. Can optionally include a Key / Type entry.

## Implementation

The implementation is in `src/operations/parse_tlv.rs` and declares `Bytes` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Parse TLV"
```

For file or binary input use `rxchef run "Parse TLV" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Parse TLV" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/parse_tlv.rs

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
