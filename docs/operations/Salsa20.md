# Salsa20

## Overview

Salsa20 is a stream cipher designed by Daniel J. Bernstein and submitted to the eSTREAM project; Salsa20/8 and Salsa20/12 are round-reduced variants. It is closely related to the ChaCha stream cipher.<br><br><b>Key:</b> Salsa20 uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> Salsa20 uses a nonce of 8 bytes (64 bits).<br><br><b>Counter:</b> Salsa uses a counter of 8 bytes (64 bits). The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes.

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

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Key (16 or 32 bytes) |
| 2 | Nonce | `Bytes` | no | `<empty>` | — | no | Nonce (8 bytes) |
| 3 | Counter | `UnsignedInteger` | no | `0` | — | no | Initial counter value |
| 4 | Rounds | `Enum` | no | `20` | 20, 12, 8 | no | Number of rounds (20, 12, or 8) |
| 5 | Input | `Enum` | no | `Raw` | Raw, Hex | no | Input format (Raw, Hex) |
| 6 | Output | `Enum` | no | `Raw` | Raw, Hex | no | Output format (Raw, Hex) |

## How it works

Salsa20 is a stream cipher designed by Daniel J. Bernstein and submitted to the eSTREAM project; Salsa20/8 and Salsa20/12 are round-reduced variants. It is closely related to the ChaCha stream cipher.<br><br><b>Key:</b> Salsa20 uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> Salsa20 uses a nonce of 8 bytes (64 bits).<br><br><b>Counter:</b> Salsa uses a counter of 8 bytes (64 bits). The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes.

## Implementation

The implementation is in `src/operations/salsa20.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Salsa20"
```

For file or binary input use `rxchef run "Salsa20" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Salsa20" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/salsa20.rs

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
