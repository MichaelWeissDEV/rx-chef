# Lorenz

## Overview

The Lorenz SZ40/42 cipher attachment was a WW2 German rotor cipher machine.

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
| 1 | Model | `String` | no | `SZ40` | — | no | SZ40, SZ42a, SZ42b |
| 2 | Wheel Pattern | `String` | no | `KH Pattern` | — | no | KH, ZMUG, BREAM, etc. |
| 3 | KT-Schalter | `Boolean` | no | `false` | — | no | Enable the SZ42a Klartext feedback switch |
| 4 | Mode | `String` | no | `Send` | — | no | Send or Receive |
| 5 | Input Type | `String` | no | `Plaintext` | — | no | Plaintext or ITA2 |
| 6 | Output Type | `String` | no | `Plaintext` | — | no | Plaintext or ITA2 |
| 7 | ITA2 Format | `String` | no | `5/8/9` | — | no | 5/8/9 or +/-/. |
| 8 | Psi1 start | `Integer` | no | `1` | — | no | 1-43 |
| 9 | Psi2 start | `Integer` | no | `1` | — | no | 1-47 |
| 10 | Psi3 start | `Integer` | no | `1` | — | no | 1-51 |
| 11 | Psi4 start | `Integer` | no | `1` | — | no | 1-53 |
| 12 | Psi5 start | `Integer` | no | `1` | — | no | 1-59 |
| 13 | Mu37 start | `Integer` | no | `1` | — | no | 1-37 |
| 14 | Mu61 start | `Integer` | no | `1` | — | no | 1-61 |
| 15 | Chi1 start | `Integer` | no | `1` | — | no | 1-41 |
| 16 | Chi2 start | `Integer` | no | `1` | — | no | 1-31 |
| 17 | Chi3 start | `Integer` | no | `1` | — | no | 1-29 |
| 18 | Chi4 start | `Integer` | no | `1` | — | no | 1-26 |
| 19 | Chi5 start | `Integer` | no | `1` | — | no | 1-23 |
| 20 | Psi1 lugs | `String` | no | `.x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x` | — | no | 43 long |
| 21 | Psi2 lugs | `String` | no | `.xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x` | — | no | 47 long |
| 22 | Psi3 lugs | `String` | no | `.x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x` | — | no | 51 long |
| 23 | Psi4 lugs | `String` | no | `.xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.` | — | no | 53 long |
| 24 | Psi5 lugs | `String` | no | `xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.` | — | no | 59 long |
| 25 | Mu37 lugs | `String` | no | `x.x.x.x.x.x...x.x.x...x.x.x...x.x....` | — | no | 37 long |
| 26 | Mu61 lugs | `String` | no | `.xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...` | — | no | 61 long |
| 27 | Chi1 lugs | `String` | no | `.x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..` | — | no | 41 long |
| 28 | Chi2 lugs | `String` | no | `x..xxx...x.xxxx..xx..x..xx.xx..` | — | no | 31 long |
| 29 | Chi3 lugs | `String` | no | `..xx..x.xxx...xx...xx..xx.xx.` | — | no | 29 long |
| 30 | Chi4 lugs | `String` | no | `xx..x..xxxx..xx.xxx....x..` | — | no | 26 long |
| 31 | Chi5 lugs | `String` | no | `xx..xx....xxxx.x..x.x..` | — | no | 23 long |

## Implementation

The implementation is in `src/operations/lorenz.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Lorenz" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/lorenz.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
