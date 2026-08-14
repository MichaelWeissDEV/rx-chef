# Triple DES Decrypt

## Overview

Decrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Hex or Raw.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Decryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64 |
| 2 | Key encoding | `String` | no | `Hex` | — | no | Encoding of the key: Hex, UTF8, Latin1, Base64 |
| 3 | IV | `Bytes` | no | `<empty>` | — | no | Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64 |
| 4 | IV encoding | `String` | no | `Hex` | — | no | Encoding of the IV: Hex, UTF8, Latin1, Base64 |
| 5 | Mode | `String` | no | `CBC` | — | no | Cipher mode: CBC, ECB, CBC/NoPadding, ECB/NoPadding |
| 6 | Input | `String` | no | `Hex` | — | no | Input encoding: Hex, Raw |
| 7 | Output | `String` | no | `Raw` | — | no | Output encoding: Raw, Hex |

## Implementation

The implementation is in `src/operations/triple_des_decrypt.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Triple DES Decrypt" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/triple_des_decrypt.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
