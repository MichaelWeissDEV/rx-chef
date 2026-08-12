# CSV to JSON

Converts a CSV file to JSON format. The first row is used as the header for 'Array of dictionaries' format. Supports quoted fields with embedded delimiters and escaped double-quotes.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "CSV to JSON"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Cell delimiter | `,` | Character used to separate fields |
| 2 | Row delimiter | ` ` | Character(s) used to separate rows |
| 3 | Format | `Array of dictionaries` | Output format: 'Array of dictionaries' or 'Array of arrays' |

