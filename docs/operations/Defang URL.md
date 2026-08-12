# Defang URL

Takes a Universal Resource Locator (URL) and 'Defangs' it; meaning the URL becomes invalid, neutralising the risk of accidentally clicking on a malicious link.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Defang URL"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Escape dots | `true` | Escape dots in the URL |
| 2 | Escape http | `true` | Escape http/https in the URL |
| 3 | Escape :// | `true` | Escape :// in the URL |
| 4 | Process | `Valid domains and full URLs` | Process option |

