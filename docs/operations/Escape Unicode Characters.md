# Escape Unicode Characters

Converts characters to their unicode-escaped notations.<br><br>Supports the prefixes:<ul><li><code>\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code></code> becomes <code>\u03C3\u03BF\u03C5</code>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Escape Unicode Characters"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Prefix | `\\u` | The prefix to use for each escape sequence |
| 2 | Encode all chars | `false` | If true, all characters will be escaped. If false, only non-printable ASCII characters will be escaped. |
| 3 | Padding | `4` | The number of hex digits to pad to |
| 4 | Uppercase hex | `true` | Whether to use uppercase hex digits |

