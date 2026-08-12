# Swap endianness

Switches the data from big-endian to little-endian or vice-versa. Data can be read in as hexadecimal or raw bytes. It will be returned in the same format as it is entered.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Swap endianness"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Data format | `Raw` | Input/output format: Hex or Raw |
| 2 | Word length (bytes) | `4` | Number of bytes per word |
| 3 | Pad incomplete words | `true` | If true, pad incomplete words with zero bytes |

