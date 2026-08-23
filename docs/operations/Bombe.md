# Bombe

## Overview

Emulation of the Bombe machine used at Bletchley Park to attack Enigma.

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

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Model | `String` | no | `3-rotor` | — | no | 3-rotor or 4-rotor |
| 2 | Left-most (4th) rotor | `String` | no | `LEYJVCNIXWPBQMDRTAKZGFUHOS` | — | no | Wiring for the 4th rotor |
| 3 | Left-hand rotor | `String` | no | `EKMFLGDQVZNTOWYHXUSPAIBRCJ` | — | no | Wiring for the left-hand rotor |
| 4 | Middle rotor | `String` | no | `AJDKSIRUXBLHWTMCQGZNPYFVOE` | — | no | Wiring for the middle rotor |
| 5 | Right-hand rotor | `String` | no | `BDFHJLCPRTXVZNYEIWGAKMUSQO` | — | no | Wiring for the right-hand rotor |
| 6 | Reflector | `String` | no | `AY BR CU DH EQ FS GL IP JX KN MO TZ VW` | — | no | Reflector pairs |
| 7 | Crib | `String` | no | `<empty>` | — | no | Known plaintext |
| 8 | Crib offset | `Integer` | no | `0` | — | no | Offset of the crib in the ciphertext |
| 9 | Use checking machine | `Boolean` | no | `true` | — | no | Whether to use the checking machine |

## Implementation

The implementation is in `src/operations/bombe.rs` and declares `String` input and `JSON` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Bombe" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `JSON` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/bombe.rs
- src/operations/bombe.rs

Known-answer tests:
- tests/tests/operations/bombe.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
