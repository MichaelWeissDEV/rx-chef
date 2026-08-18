# From Bech32

## Overview

Bech32 is an encoding scheme primarily used for Bitcoin SegWit addresses (BIP-0173). It uses a 32-character alphabet that excludes easily confused characters (1, b, i, o) and includes a checksum for error detection.

Bech32m (BIP-0350) is an updated version used for Bitcoin Taproot addresses.

Auto-detect will attempt Bech32 first, then Bech32m if the checksum fails.

Output format options allow you to see the Human-Readable Part (HRP) along with the decoded data.

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
| 1 | Encoding | `String` | no | `Auto-detect` | — | no | The Bech32 encoding variant |
| 2 | Output Format | `String` | no | `Raw` | — | no | The format of the output |

## Implementation

The implementation is in `src/operations/from_bech32.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "From Bech32" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/from_bech32.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
