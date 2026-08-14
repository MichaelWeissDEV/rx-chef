# Bombe

## Overview

Emulation of the Bombe machine used at Bletchley Park to attack Enigma.

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

## How it works

Emulation of the Bombe machine used at Bletchley Park to attack Enigma.

## Implementation

The implementation is in `src/operations/bombe.rs` and declares `String` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Bombe"
```

For file or binary input use `rxchef run "Bombe" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Bombe" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/bombe.rs

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
