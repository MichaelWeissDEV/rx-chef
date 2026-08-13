# Lorenz

## Overview

The Lorenz SZ40/42 cipher attachment was a WW2 German rotor cipher machine.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
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

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/lorenz.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Lorenz"
```

For file or binary input use `rxchef run "Lorenz" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Lorenz" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
