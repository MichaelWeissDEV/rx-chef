# Pseudo-Random Integer Generator

A cryptographically-secure pseudo-random number generator (PRNG). Generates random integers within a specified range. The supported range of integers is from -(2^53 - 1) to (2^53 - 1).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Pseudo-Random Integer Generator"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Number of Integers | `1` | How many integers to generate |
| 2 | Min Value | `0` | Minimum value (inclusive) |
| 3 | Max Value | `99` | Maximum value (inclusive) |
| 4 | Delimiter | `Space` | Delimiter between integers |
| 5 | Output | `Decimal` | Output format (Raw, Hex, Decimal) |

