# Render Markdown

Renders input Markdown as HTML. HTML rendering is disabled to avoid XSS. (Simplified port using Regex)

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Render Markdown"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Autoconvert URLs to links | `false` | Autoconvert URLs to links |
| 2 | Enable syntax highlighting | `true` | Enable syntax highlighting (Not supported in this port) |
| 3 | Open links in new tab. | `false` | Adds target="_blank" to links. |

