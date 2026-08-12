# Magic

The Magic operation attempts to detect various properties of the input data and suggests which operations could help to make more sense of it.

- Input: `Bytes`
- Output: `JSON`
- CLI: `rxchef run "Magic"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Depth | `3` | Maximum number of levels of recursion |
| 2 | Intensive mode | `false` | Brute-force XOR, bit rotates, etc. |
| 3 | Extensive language support | `false` | Compare byte frequencies to a large number of languages |
| 4 | Crib (known plaintext string or regex) | `<empty>` | Filter results by matching this string or regex |

