# Typex

## Overview

Encipher/decipher with the WW2 Typex machine.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
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

Encipher/decipher with the WW2 Typex machine.

## Implementation

The implementation is in `src/operations/typex.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

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

Correctness:
- tests/tests/operations/typex.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
