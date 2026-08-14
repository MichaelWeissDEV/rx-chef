# Whirlpool

## Overview

Whirlpool is a cryptographic hash function designed by Vincent Rijmen (co-creator of AES) and Paulo S. L. M. Barreto, who first described it in 2000. Whirlpool is the latest revision, released in 2003, fixing a flaw in the diffusion matrix.

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
| 1 | Variant | `String` | no | `Whirlpool` | — | no | Hash variant (Whirlpool only) |
| 2 | Rounds | `UnsignedInteger` | no | `10` | — | no | Number of rounds (1-10) |

## How it works

Whirlpool is a cryptographic hash function designed by Vincent Rijmen (co-creator of AES) and Paulo S. L. M. Barreto, who first described it in 2000. Whirlpool is the latest revision, released in 2003, fixing a flaw in the diffusion matrix.

## Implementation

The implementation is in `src/operations/whirlpool.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Whirlpool"
```

For file or binary input use `rxchef run "Whirlpool" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Whirlpool" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/whirlpool.rs

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
