# AES Decrypt

## Overview

Advanced Encryption Standard (AES) is a U.S. Federal Information Processing Standard (FIPS). It was selected after a 5-year process where 15 competing designs were evaluated.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Required` |
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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Decryption key (16, 24, or 32 bytes) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization Vector (16 bytes, optional, defaults to null) |
| 3 | Mode | `Enum` | no | `CBC` | CBC, CFB, OFB, CTR, GCM, ECB | no | Cipher mode (CBC, CFB, OFB, CTR, GCM, ECB) |
| 4 | Input | `Enum` | no | `Hex` | Hex, Raw | no | Input encoding (Hex, Raw) |
| 5 | Output | `Enum` | no | `Raw` | Raw, Hex | no | Output encoding (Raw, Hex) |
| 6 | GCM Tag | `String` | no | `<empty>` | — | no | GCM authentication tag (optional) |
| 7 | Additional Authenticated Data | `Bytes` | no | `<empty>` | — | no | AAD for GCM mode (optional) |

## Implementation

The implementation is in `src/operations/aes_decrypt.rs` and declares `Bytes` input and `Bytes` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "AES Decrypt" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `Bytes` value is valid. Its `Bytes` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/aes_decrypt.rs
- tests/tests/known_answer_vectors.rs

Known-answer tests:
- tests/tests/known_answer_vectors.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
