# Unescape Unicode Characters

Converts unicode-escaped character notation back into raw characters.<br><br>Supports the prefixes:<ul><li><code>\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code>\u03c3\u03bf\u03c5</code> becomes <code></code>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Unescape Unicode Characters"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Prefix | `\\u` | The prefix used for the unicode escape sequence |

