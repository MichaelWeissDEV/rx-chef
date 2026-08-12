# To Morse Code

Translates alphanumeric characters into International Morse Code.<br><br>Ignores non-Morse characters.<br><br>e.g. <code>SOS</code> becomes <code>... --- ...</code>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "To Morse Code"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Format options | `-/.` | The characters to use for dots and dashes |
| 2 | Letter delimiter | `Space` | The delimiter between letters |
| 3 | Word delimiter | `Line feed` | The delimiter between words |

