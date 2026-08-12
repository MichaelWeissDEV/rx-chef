# ROT13

Rotates each letter by 13 positions in the alphabet. ROT13 is a simple Caesar cipher.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "ROT13"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Rotate lower case chars | `true` | Apply rotation to a-z |
| 2 | Rotate upper case chars | `true` | Apply rotation to A-Z |
| 3 | Rotate digits | `false` | Apply ROT5 to digits 0-9 |
| 4 | Amount | `13` | Amount to rotate (default 13) |

