# Fork

Split the input data up based on the specified delimiter and run all subsequent operations on each branch separately. In this implementation, acts as a passthrough (flow control requires recipe-level orchestration).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Fork"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Split delimiter | `\\n` | The delimiter to split the input on |
| 2 | Merge delimiter | `\\n` | The delimiter to join outputs with |
| 3 | Ignore errors | `false` | Continue processing even if a branch fails |

