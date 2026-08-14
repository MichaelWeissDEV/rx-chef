# Enigma

## Overview

Encipher/decipher with the WW2 Enigma machine.

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

## How it works

Encipher/decipher with the WW2 Enigma machine.

## Implementation

The implementation is in `src/operations/enigma.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Enigma"
```

For file or binary input use `rxchef run "Enigma" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Enigma" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/enigma.rs

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
