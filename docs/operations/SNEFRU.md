# SNEFRU

## Overview

Computes the standardized 256-bit, 8-round SNEFRU hash. SNEFRU was designed by Ralph Merkle in 1990; the original shorter-round design is retained in the argument schema for recipe compatibility but rejected because it is cryptographically broken.

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
| 1 | Size | `UnsignedInteger` | no | `256` | — | no | Output size in bits (supported: 256) |
| 2 | Rounds | `UnsignedInteger` | no | `8` | — | no | Number of rounds (supported: 8) |

## How it works

Computes the standardized 256-bit, 8-round SNEFRU hash. SNEFRU was designed by Ralph Merkle in 1990; the original shorter-round design is retained in the argument schema for recipe compatibility but rejected because it is cryptographically broken.

## Implementation

The implementation is in `src/operations/snefru.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SNEFRU"
```

For file or binary input use `rxchef run "SNEFRU" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SNEFRU" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/snefru.rs

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
