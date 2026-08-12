# Flask Session Verify

Verifies the HMAC signature of a Flask session cookie (itsdangerous) generated.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "Flask Session Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Secret key (UTF-8) |
| 2 | Salt | `cookie-session` | Salt string (default: cookie-session) |
| 3 | Algorithm | `sha1` | HMAC algorithm: sha1 or sha256 |
| 4 | View Timestamp | `true` | Include timestamp in output |

