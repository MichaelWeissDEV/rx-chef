# URL Encode

Encodes problematic characters into percent-encoding, a format supported by URIs/URLs.<br><br>e.g. <code>=</code> becomes <code>%3d</code>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "URL Encode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Encode all special chars | `false` | Encode all characters including those usually allowed in URLs |

