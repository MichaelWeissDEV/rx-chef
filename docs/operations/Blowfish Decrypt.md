# Blowfish Decrypt

## Overview

Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier and included in a large number of cipher suites and encryption products. AES now receives more attention.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Decryption key (4-56 bytes) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization Vector (8 bytes for non-ECB modes, optional) |
| 3 | Mode | `Enum` | no | `CBC` | CBC, CFB, OFB, CTR, ECB | no | Cipher mode (CBC, CFB, OFB, CTR, ECB) |
| 4 | Input | `Enum` | no | `Hex` | Hex, Raw | no | Input encoding (Hex, Raw) |
| 5 | Output | `Enum` | no | `Raw` | Raw, Hex | no | Output encoding (Raw, Hex) |

## Implementation

The implementation is in `src/operations/blowfish_decrypt.rs` and declares `Bytes` input and `Bytes` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Blowfish Decrypt" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/blowfish_decrypt.rs

Known-answer tests:
- tests/tests/operations/blowfish_decrypt.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
