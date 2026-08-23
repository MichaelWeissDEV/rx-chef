# GOST Verify

## Overview

Verify the signature of a plaintext message using one of the GOST block ciphers. Enter the signature in the MAC field.

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

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | The decryption key. |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | The initialization vector. |
| 3 | MAC | `String` | no | `<empty>` | — | no | The signature/MAC to verify. |
| 4 | Input type | `String` | no | `Raw` | — | no | Type of input data |
| 5 | Algorithm | `String` | no | `GOST R 34.12 (Magma, 2015)` | — | no | GOST version |
| 6 | sBox | `String` | no | `E-TEST` | — | no | S-Box to use (1989 only) |

## Implementation

The implementation is in `src/operations/gost_verify.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "GOST Verify" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/gost_verify.rs

Known-answer tests:
- tests/tests/operations/gost_verify.rs

Differential tests:
- tests/tests/operations/gost_verify.rs
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
