# JavaScript Parser

Parses JavaScript and returns a SWC Abstract Syntax Tree as JSON. Optional source locations, byte ranges, tokens, comments, and recoverable parser errors can be included.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "JavaScript Parser"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Location info | `false` | Include line and column location information |
| 2 | Range info | `false` | Include range information |
| 3 | Include tokens array | `false` | Include tokens array |
| 4 | Include comments array | `false` | Include comments array |
| 5 | Report errors and try to continue | `false` | Report errors and try to continue |

