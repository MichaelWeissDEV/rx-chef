# SIGABA

## Overview

Encipher/decipher with the WW2 SIGABA machine. <br><br>SIGABA, otherwise known as ECM Mark II, was used by the United States for message encryption during WW2 up to the 1950s. It was developed in the 1930s by the US Army and Navy, and has up to this day never been broken. Consisting of 15 rotors: 5 cipher rotors and 10 rotors (5 control rotors and 5 index rotors) controlling the stepping of the cipher rotors, the rotor stepping for SIGABA is much more complex than other rotor machines of its time, such as Enigma. All example rotor wirings are random example sets.<br><br>To configure rotor wirings, for the cipher and control rotors enter a string of letters which map from A to Z, and for the index rotors enter a sequence of numbers which map from 0 to 9. Note that encryption is not the same as decryption, so first choose the desired mode. <br><br> Note: Whilst this has been tested against other software emulators, it has not been tested against hardware.

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
| 1 | 1st cipher rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 2 | 1st cipher rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 3 | 1st cipher rotor initial value | `String` | no | `A` | — | no | Initial value |
| 4 | 2nd cipher rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 5 | 2nd cipher rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 6 | 2nd cipher rotor initial value | `String` | no | `A` | — | no | Initial value |
| 7 | 3rd cipher rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 8 | 3rd cipher rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 9 | 3rd cipher rotor initial value | `String` | no | `A` | — | no | Initial value |
| 10 | 4th cipher rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 11 | 4th cipher rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 12 | 4th cipher rotor initial value | `String` | no | `A` | — | no | Initial value |
| 13 | 5th cipher rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 14 | 5th cipher rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 15 | 5th cipher rotor initial value | `String` | no | `A` | — | no | Initial value |
| 16 | 1st control rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 17 | 1st control rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 18 | 1st control rotor initial value | `String` | no | `A` | — | no | Initial value |
| 19 | 2nd control rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 20 | 2nd control rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 21 | 2nd control rotor initial value | `String` | no | `A` | — | no | Initial value |
| 22 | 3rd control rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 23 | 3rd control rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 24 | 3rd control rotor initial value | `String` | no | `A` | — | no | Initial value |
| 25 | 4th control rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 26 | 4th control rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 27 | 4th control rotor initial value | `String` | no | `A` | — | no | Initial value |
| 28 | 5th control rotor | `String` | no | `SRGWANHPJZFXVIDQCEUKBYOLMT` | — | no | Rotor wiring |
| 29 | 5th control rotor reversed | `Boolean` | no | `false` | — | no | Reversed orientation |
| 30 | 5th control rotor initial value | `String` | no | `A` | — | no | Initial value |
| 31 | 1st index rotor | `Integer` | no | `6201348957` | — | no | Rotor wiring |
| 32 | 1st index rotor initial value | `Integer` | no | `0` | — | no | Initial value |
| 33 | 2nd index rotor | `Integer` | no | `6201348957` | — | no | Rotor wiring |
| 34 | 2nd index rotor initial value | `Integer` | no | `0` | — | no | Initial value |
| 35 | 3rd index rotor | `Integer` | no | `6201348957` | — | no | Rotor wiring |
| 36 | 3rd index rotor initial value | `Integer` | no | `0` | — | no | Initial value |
| 37 | 4th index rotor | `Integer` | no | `6201348957` | — | no | Rotor wiring |
| 38 | 4th index rotor initial value | `Integer` | no | `0` | — | no | Initial value |
| 39 | 5th index rotor | `Integer` | no | `6201348957` | — | no | Rotor wiring |
| 40 | 5th index rotor initial value | `Integer` | no | `0` | — | no | Initial value |
| 41 | SIGABA mode | `String` | no | `Encrypt` | — | no | Encrypt or Decrypt |

## How it works

Encipher/decipher with the WW2 SIGABA machine. <br><br>SIGABA, otherwise known as ECM Mark II, was used by the United States for message encryption during WW2 up to the 1950s. It was developed in the 1930s by the US Army and Navy, and has up to this day never been broken. Consisting of 15 rotors: 5 cipher rotors and 10 rotors (5 control rotors and 5 index rotors) controlling the stepping of the cipher rotors, the rotor stepping for SIGABA is much more complex than other rotor machines of its time, such as Enigma. All example rotor wirings are random example sets.<br><br>To configure rotor wirings, for the cipher and control rotors enter a string of letters which map from A to Z, and for the index rotors enter a sequence of numbers which map from 0 to 9. Note that encryption is not the same as decryption, so first choose the desired mode. <br><br> Note: Whilst this has been tested against other software emulators, it has not been tested against hardware.

## Implementation

The implementation is in `src/operations/sigaba.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SIGABA"
```

For file or binary input use `rxchef run "SIGABA" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SIGABA" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/sigaba.rs

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
