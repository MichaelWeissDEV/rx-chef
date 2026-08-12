# To Table

Data can be split on different characters and rendered as an HTML, ASCII or Markdown table with an optional header row.<br><br>Supports the CSV (Comma Separated Values) file format by default. Change the cell delimiter argument to <code>\t</code> to support TSV (Tab Separated Values) or <code>|</code> for PSV (Pipe Separated Values).<br><br>You can enter as many delimiters as you like. Each character will be treat as a separate possible delimiter.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "To Table"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Cell delimiters | `,` | Characters used to separate cells |
| 2 | Row delimiters | `\\r\\n` | Characters used to separate rows |
| 3 | Make first row header | `false` | Treat the first row as a header row |
| 4 | Format | `ASCII` | The output format |

