# Keccak

## Overview

The Keccak hash algorithm was designed by Guido Bertoni, Joan Daemen, Michael Peeters, and Gilles Van Assche, building upon RadioGatan. It was selected as the winner of the SHA-3 design competition. This version of the algorithm is Keccak[c=2d] and differs from the SHA-3 specification.

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
| 1 | Size | `UnsignedInteger` | no | `256` | — | no | Output size in bits: 512, 384, 256, or 224 |

## How it works

The Keccak hash algorithm was designed by Guido Bertoni, Joan Daemen, Michael Peeters, and Gilles Van Assche, building upon RadioGatan. It was selected as the winner of the SHA-3 design competition. This version of the algorithm is Keccak[c=2d] and differs from the SHA-3 specification.

## Implementation

The implementation is in `src/operations/keccak.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Keccak"
```

For file or binary input use `rxchef run "Keccak" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Keccak" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/keccak.rs

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
