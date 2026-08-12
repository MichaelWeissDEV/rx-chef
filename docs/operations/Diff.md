# Diff

Compares two inputs (separated by the specified delimiter) and highlights the differences between them.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Diff"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Sample delimiter | `\\n\\n` | Delimiter separating the two input samples |
| 2 | Show added | `true` | Show added text (wrapped in <ins> tags) |
| 3 | Show removed | `true` | Show removed text (wrapped in <del> tags) |
| 4 | Show subtraction | `false` | Show unchanged text |
| 5 | Ignore whitespace | `false` | Ignore leading/trailing whitespace when comparing |
| 6 | Ignore case | `false` | Perform case-insensitive comparison |

