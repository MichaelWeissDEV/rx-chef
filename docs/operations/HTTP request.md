# HTTP request

Makes an HTTP request and returns the response.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "HTTP request"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Method | `GET` | HTTP method |
| 2 | URL | `<empty>` | The URL to request |
| 3 | Headers | `<empty>` | Request headers (Key: Value) |
| 4 | Mode | `Cross-Origin Resource Sharing` | CORS mode (ignored in Rust) |
| 5 | Show response metadata | `false` | Include status and headers in output |

