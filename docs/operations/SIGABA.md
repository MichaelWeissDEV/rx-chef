# SIGABA

Encipher/decipher with the WW2 SIGABA machine. <br><br>SIGABA, otherwise known as ECM Mark II, was used by the United States for message encryption during WW2 up to the 1950s. It was developed in the 1930s by the US Army and Navy, and has up to this day never been broken. Consisting of 15 rotors: 5 cipher rotors and 10 rotors (5 control rotors and 5 index rotors) controlling the stepping of the cipher rotors, the rotor stepping for SIGABA is much more complex than other rotor machines of its time, such as Enigma. All example rotor wirings are random example sets.<br><br>To configure rotor wirings, for the cipher and control rotors enter a string of letters which map from A to Z, and for the index rotors enter a sequence of numbers which map from 0 to 9. Note that encryption is not the same as decryption, so first choose the desired mode. <br><br> Note: Whilst this has been tested against other software emulators, it has not been tested against hardware.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "SIGABA"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | 1st cipher rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 2 | 1st cipher rotor reversed | `false` | Reversed orientation |
| 3 | 1st cipher rotor initial value | `A` | Initial value |
| 4 | 2nd cipher rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 5 | 2nd cipher rotor reversed | `false` | Reversed orientation |
| 6 | 2nd cipher rotor initial value | `A` | Initial value |
| 7 | 3rd cipher rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 8 | 3rd cipher rotor reversed | `false` | Reversed orientation |
| 9 | 3rd cipher rotor initial value | `A` | Initial value |
| 10 | 4th cipher rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 11 | 4th cipher rotor reversed | `false` | Reversed orientation |
| 12 | 4th cipher rotor initial value | `A` | Initial value |
| 13 | 5th cipher rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 14 | 5th cipher rotor reversed | `false` | Reversed orientation |
| 15 | 5th cipher rotor initial value | `A` | Initial value |
| 16 | 1st control rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 17 | 1st control rotor reversed | `false` | Reversed orientation |
| 18 | 1st control rotor initial value | `A` | Initial value |
| 19 | 2nd control rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 20 | 2nd control rotor reversed | `false` | Reversed orientation |
| 21 | 2nd control rotor initial value | `A` | Initial value |
| 22 | 3rd control rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 23 | 3rd control rotor reversed | `false` | Reversed orientation |
| 24 | 3rd control rotor initial value | `A` | Initial value |
| 25 | 4th control rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 26 | 4th control rotor reversed | `false` | Reversed orientation |
| 27 | 4th control rotor initial value | `A` | Initial value |
| 28 | 5th control rotor | `SRGWANHPJZFXVIDQCEUKBYOLMT` | Rotor wiring |
| 29 | 5th control rotor reversed | `false` | Reversed orientation |
| 30 | 5th control rotor initial value | `A` | Initial value |
| 31 | 1st index rotor | `6201348957` | Rotor wiring |
| 32 | 1st index rotor initial value | `0` | Initial value |
| 33 | 2nd index rotor | `6201348957` | Rotor wiring |
| 34 | 2nd index rotor initial value | `0` | Initial value |
| 35 | 3rd index rotor | `6201348957` | Rotor wiring |
| 36 | 3rd index rotor initial value | `0` | Initial value |
| 37 | 4th index rotor | `6201348957` | Rotor wiring |
| 38 | 4th index rotor initial value | `0` | Initial value |
| 39 | 5th index rotor | `6201348957` | Rotor wiring |
| 40 | 5th index rotor initial value | `0` | Initial value |
| 41 | SIGABA mode | `Encrypt` | Encrypt or Decrypt |

