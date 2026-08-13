# Fork

Split the input on the specified delimiter and run subsequent recipe operations on each branch separately until Merge. This flow-control operation is interpreted by integration::bake and all CLI recipe frontends.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Fork"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Split delimiter | `\\n` | The delimiter to split the input on |
| 2 | Merge delimiter | `\\n` | The delimiter to join outputs with |
| 3 | Ignore errors | `false` | Continue processing even if a branch fails |

