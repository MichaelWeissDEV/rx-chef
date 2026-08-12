# Subsection

Select a part of the input data using a regular expression (regex), and run all subsequent operations on each match separately. In this implementation, acts as a passthrough (flow control requires recipe-level orchestration).

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

