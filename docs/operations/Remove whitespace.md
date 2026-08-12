# Remove whitespace

Optionally removes all spaces, carriage returns, line feeds, tabs and form feeds from the input data. This operation also supports the removal of full stops which are sometimes used to represent non-printable bytes in ASCII output.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Remove whitespace"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Spaces | `true` | Remove spaces |
| 2 | Carriage returns (\\r) | `true` | Remove carriage returns |
| 3 | Line feeds (\\n) | `true` | Remove line feeds |
| 4 | Tabs | `true` | Remove tabs |
| 5 | Form feeds (\\f) | `true` | Remove form feeds |
| 6 | Full stops | `false` | Remove full stops |

