# Triple DES Decrypt

## Overview

Decrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Hex or Raw.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Decryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64 |
| 2 | Key encoding | `String` | no | `Hex` | — | no | Encoding of the key: Hex, UTF8, Latin1, Base64 |
| 3 | IV | `Bytes` | no | `<empty>` | — | no | Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64 |
| 4 | IV encoding | `String` | no | `Hex` | — | no | Encoding of the IV: Hex, UTF8, Latin1, Base64 |
| 5 | Mode | `String` | no | `CBC` | — | no | Cipher mode: CBC, ECB, CBC/NoPadding, ECB/NoPadding |
| 6 | Input | `String` | no | `Hex` | — | no | Input encoding: Hex, Raw |
| 7 | Output | `String` | no | `Raw` | — | no | Output encoding: Raw, Hex |

## How it works

Decrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Hex or Raw.

## Implementation

The implementation is in `src/operations/triple_des_decrypt.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Triple DES Decrypt"
```

For file or binary input use `rxchef run "Triple DES Decrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Triple DES Decrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/triple_des_decrypt.rs

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
