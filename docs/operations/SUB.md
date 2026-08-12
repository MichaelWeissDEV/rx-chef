# SUB

SUB the input with the given key (e.g. fe023da5), MOD 256

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "SUB"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Key to SUB from input bytes |
| 2 | Scheme | `Standard` | Scheme (Standard, Input differential, Output differential) |
| 3 | Null preserving | `false` | If true, bytes that are 0 or equal to the key byte are not modified |

