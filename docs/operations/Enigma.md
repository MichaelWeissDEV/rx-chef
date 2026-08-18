# Enigma

## Overview

Encipher/decipher with the WW2 Enigma machine.

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
| 1 | Model | `String` | no | `3-rotor` | — | no | 3-rotor or 4-rotor |
| 2 | 4th Rotor | `String` | no | `<empty>` | — | no | Wiring<Steps |
| 3 | 4th Ring | `String` | no | `A` | — | no | A-Z |
| 4 | 4th Pos | `String` | no | `A` | — | no | A-Z |
| 5 | Left Rotor | `String` | no | `EKMFLGDQVZNTOWYHXUSPAIBRCJ<R` | — | no | Wiring<Steps |
| 6 | Left Ring | `String` | no | `A` | — | no | A-Z |
| 7 | Left Pos | `String` | no | `A` | — | no | A-Z |
| 8 | Middle Rotor | `String` | no | `AJDKSIRUXBLHWTMCQGZNPYFVOE<F` | — | no | Wiring<Steps |
| 9 | Middle Ring | `String` | no | `A` | — | no | A-Z |
| 10 | Middle Pos | `String` | no | `A` | — | no | A-Z |
| 11 | Right Rotor | `String` | no | `BDFHJLCPRTXVZNYEIWGAKMUSQO<W` | — | no | Wiring<Steps |
| 12 | Right Ring | `String` | no | `A` | — | no | A-Z |
| 13 | Right Pos | `String` | no | `A` | — | no | A-Z |
| 14 | Reflector | `String` | no | `AY BR CU DH EQ FS GL IP JX KN MO TZ VW` | — | no | Pairs |
| 15 | Plugboard | `String` | no | `<empty>` | — | no | Pairs |
| 16 | Strict | `Boolean` | no | `true` | — | no | Boolean |

## Implementation

The implementation is in `src/operations/enigma.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Enigma" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/enigma.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
