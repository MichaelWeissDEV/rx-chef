# Multiple Bombe

Emulation of the Bombe machine used to attack Enigma. This version carries out multiple Bombe runs to handle unknown rotor configurations.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "Multiple Bombe"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Standard Enigmas | `German Service Enigma (First - 3 rotor)` | Preset rotor configurations |
| 2 | Main rotors | `<empty>` | Newline separated rotor wirings |
| 3 | 4th rotor | `<empty>` | Newline separated 4th rotor wirings |
| 4 | Reflectors | `<empty>` | Newline separated reflector pairs |
| 5 | Crib | `<empty>` | Known plaintext |
| 6 | Crib offset | `0` | Offset of the crib in the ciphertext |
| 7 | Use checking machine | `true` | Whether to use the checking machine |

