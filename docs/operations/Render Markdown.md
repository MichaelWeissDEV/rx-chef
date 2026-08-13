# Render Markdown

Renders Markdown as safe HTML. Raw HTML is escaped, URLs can be linked automatically, fenced code blocks can be syntax-highlighted, and links can open in a new tab.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Render Markdown"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Autoconvert URLs to links | `false` | Autoconvert URLs to links |
| 2 | Enable syntax highlighting | `true` | Highlight strings, numbers, comments, and common language keywords in fenced code blocks |
| 3 | Open links in new tab. | `false` | Adds target="_blank" to links. |

