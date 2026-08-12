# Entropy

Shannon Entropy, in the context of information theory, is a measure of the rate at which information is produced by a source of data. 8 is the maximum, representing highly unstructured, random data. English language text usually falls somewhere between 3.5 and 5. Properly encrypted or compressed data should have an entropy of over 7.5.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Entropy"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Chunk size | `0` | Size of each chunk for scanning entropy. 0 means calculate for whole input. |

