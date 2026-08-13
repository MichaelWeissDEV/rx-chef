# Jump

Jump forwards or backwards to a Label in a recipe. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Jump"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Label name | `<empty>` | The name of the label to jump to |
| 2 | Maximum jumps (if jumping backwards) | `10` | The maximum number of times to jump backwards to prevent infinite loops |

