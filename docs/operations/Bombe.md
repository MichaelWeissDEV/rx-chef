# Bombe

Emulation of the Bombe machine used at Bletchley Park to attack Enigma.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "Bombe"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Model | `3-rotor` | 3-rotor or 4-rotor |
| 2 | Left-most (4th) rotor | `LEYJVCNIXWPBQMDRTAKZGFUHOS` | Wiring for the 4th rotor |
| 3 | Left-hand rotor | `EKMFLGDQVZNTOWYHXUSPAIBRCJ` | Wiring for the left-hand rotor |
| 4 | Middle rotor | `AJDKSIRUXBLHWTMCQGZNPYFVOE` | Wiring for the middle rotor |
| 5 | Right-hand rotor | `BDFHJLCPRTXVZNYEIWGAKMUSQO` | Wiring for the right-hand rotor |
| 6 | Reflector | `AY BR CU DH EQ FS GL IP JX KN MO TZ VW` | Reflector pairs |
| 7 | Crib | `<empty>` | Known plaintext |
| 8 | Crib offset | `0` | Offset of the crib in the ciphertext |
| 9 | Use checking machine | `true` | Whether to use the checking machine |

