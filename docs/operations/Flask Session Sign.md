# Flask Session Sign

Signs a JSON payload to produce a Flask session cookie (itsdangerous HMAC).

- Input: `JSON`
- Output: `String`
- CLI: `rxchef run "Flask Session Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Secret key (UTF-8) |
| 2 | Salt | `cookie-session` | Salt string (default: cookie-session) |
| 3 | Algorithm | `sha1` | HMAC algorithm: sha1 or sha256 |

