# ROT47 Brute Force

Try all meaningful amounts for ROT47. Optionally you can enter your known plaintext (crib) to filter the result.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "ROT47 Brute Force"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Sample length | `100` | Number of bytes to sample from input |
| 2 | Sample offset | `0` | Byte offset to start sampling |
| 3 | Print amount | `true` | Prefix each result with its rotation amount (true/false) |
| 4 | Crib (known plaintext string) | `<empty>` | Filter results to those containing this string |

