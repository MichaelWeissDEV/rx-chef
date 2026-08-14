# BLAKE2b

## Overview

Performs BLAKE2b hashing on the input. BLAKE2b is a flavour of the BLAKE cryptographic hash function that is optimized for 64-bit platforms and produces digests of any size between 1 and 64 bytes. Supports the use of an optional key.

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
| 1 | Size | `Enum` | no | `512` | 512, 384, 256, 160, 128 | no | Output size in bits (512, 384, 256, 160, 128) |
| 2 | Output Encoding | `Enum` | no | `Hex` | Hex, Base64, Raw | no | Output encoding (Hex, Base64, Raw) |
| 3 | Key | `Bytes` | yes | `<empty>` | — | yes | Optional key for keyed hashing |

## How it works

Performs BLAKE2b hashing on the input. BLAKE2b is a flavour of the BLAKE cryptographic hash function that is optimized for 64-bit platforms and produces digests of any size between 1 and 64 bytes. Supports the use of an optional key.

## Implementation

The implementation is in `src/operations/blake2b.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "BLAKE2b"
```

For file or binary input use `rxchef run "BLAKE2b" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "BLAKE2b" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/blake2b.rs

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
