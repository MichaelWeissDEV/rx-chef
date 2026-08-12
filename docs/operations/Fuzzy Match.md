# Fuzzy Match

Conducts a fuzzy search to find a pattern within the input based on weighted criteria.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Fuzzy Match"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Search | `<empty>` | Pattern to search for |
| 2 | Sequential bonus | `15` | Bonus for adjacent matches |
| 3 | Separator bonus | `30` | Bonus if match occurs after a separator |
| 4 | Camel bonus | `30` | Bonus if match is uppercase and previous is lower |
| 5 | First letter bonus | `15` | Bonus if the first letter is matched |
| 6 | Leading letter penalty | `-5` | Penalty applied for every letter in the input before the first match |
| 7 | Max leading letter penalty | `-15` | Maxiumum penalty for leading letters |
| 8 | Unmatched letter penalty | `-1` | Unmatched letter penalty |

