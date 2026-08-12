# Affine Cipher Encode

The Affine cipher is a type of monoalphabetic substitution cipher, wherein each letter in an alphabet is mapped to its numeric equivalent, encrypted using a simple mathematical function, (ax + b) % 26, and converted back to a letter.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Affine Cipher Encode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | a | `1` | Multiplier parameter (must be coprime to 26) |
| 2 | b | `0` | Shift parameter |

