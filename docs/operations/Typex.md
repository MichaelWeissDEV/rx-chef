# Typex

## Overview

Encipher/decipher with the WW2 Typex machine.

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
| 1 | 1st rotor | `String` | no | `MCYLPQUVRXGSAOWNBJEZDTFKHI<BFHNQUW` | — | no | Wiring<Steps |
| 2 | 1st rotor reversed | `Boolean` | no | `false` | — | no | Boolean |
| 3 | 1st rotor ring setting | `String` | no | `A` | — | no | A-Z |
| 4 | 1st rotor initial value | `String` | no | `A` | — | no | A-Z |
| 5 | 2nd rotor | `String` | no | `KHWENRCBISXJQGOFMAPVYZDLTU<BFHNQUW` | — | no | Wiring<Steps |
| 6 | 2nd rotor reversed | `Boolean` | no | `false` | — | no | Boolean |
| 7 | 2nd rotor ring setting | `String` | no | `A` | — | no | A-Z |
| 8 | 2nd rotor initial value | `String` | no | `A` | — | no | A-Z |
| 9 | 3rd rotor | `String` | no | `BYPDZMGIKQCUSATREHOJNLFWXV<BFHNQUW` | — | no | Wiring<Steps |
| 10 | 3rd rotor reversed | `Boolean` | no | `false` | — | no | Boolean |
| 11 | 3rd rotor ring setting | `String` | no | `A` | — | no | A-Z |
| 12 | 3rd rotor initial value | `String` | no | `A` | — | no | A-Z |
| 13 | 4th rotor | `String` | no | `ZANJCGDLVHIXOBRPMSWQUKFYET<BFHNQUW` | — | no | Wiring<Steps |
| 14 | 4th rotor reversed | `Boolean` | no | `false` | — | no | Boolean |
| 15 | 4th rotor ring setting | `String` | no | `A` | — | no | A-Z |
| 16 | 4th rotor initial value | `String` | no | `A` | — | no | A-Z |
| 17 | 5th rotor | `String` | no | `QXBGUTOVFCZPJIHSWERYNDAMLK<BFHNQUW` | — | no | Wiring<Steps |
| 18 | 5th rotor reversed | `Boolean` | no | `false` | — | no | Boolean |
| 19 | 5th rotor ring setting | `String` | no | `A` | — | no | A-Z |
| 20 | 5th rotor initial value | `String` | no | `A` | — | no | A-Z |
| 21 | Reflector | `String` | no | `AN BC FG IE KD LU MH OR TS VZ WQ XJ YP` | — | no | Pairs |
| 22 | Plugboard | `String` | no | `ABCDEFGHIJKLMNOPQRSTUVWXYZ` | — | no | A-Z (26 chars) |
| 23 | Typex keyboard emulation | `String` | no | `None` | — | no | None, Encrypt, Decrypt |
| 24 | Strict output | `Boolean` | no | `true` | — | no | Boolean |

## Implementation

The implementation is in `src/operations/typex.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Typex" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/typex.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
