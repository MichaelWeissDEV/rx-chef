# XOR Brute Force

Enumerate all possible XOR solutions. Optionally enter a string that you expect to find in the plaintext to filter results (crib).

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "XOR Brute Force"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key length | `1` | Length of the XOR key in bytes (1..=2 recommended) |
| 2 | Sample length | `100` | Number of bytes of input to process |
| 3 | Sample offset | `0` | Byte offset to start sampling from |
| 4 | Scheme | `Standard` | Standard, Input differential, or Output differential |
| 5 | Null preserving | `false` | Do not XOR null bytes or bytes equal to the key |
| 6 | Print key | `true` | Prefix each result with the key used |
| 7 | Output as hex | `false` | Output results as hex instead of text |
| 8 | Crib (known plaintext string) | `<empty>` | Filter results to those containing this string |

