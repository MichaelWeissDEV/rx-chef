# Subsection

Select input sections using a regular expression and run subsequent recipe operations on every match until Merge. Non-matching bytes remain unchanged. Interpreted by integration::bake and all CLI recipe frontends.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Subsection"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Section (regex) | `<empty>` | The regex to select the subsection with |
| 2 | Case sensitive matching | `true` | Whether the regex match should be case sensitive |
| 3 | Global matching | `true` | Whether to match all occurrences |
| 4 | Ignore errors | `false` | Whether to ignore errors in subsequent operations |

