# Fang URL

Takes a 'Defanged' Universal Resource Locator (URL) and 'Fangs' it. Meaning, it removes the alterations (defanged) that render it useless so that it can be used again.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Fang URL"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Restore [.] | `true` | Restore [.] to . |
| 2 | Restore hxxp | `true` | Restore hxxp to http |
| 3 | Restore :// | `true` | Restore [://] to :// |

