# Typex

## Overview

Encipher/decipher with the WW2 Typex machine.

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

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/typex.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Typex"
```

For file or binary input use `rxchef run "Typex" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Typex" to_base64
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
