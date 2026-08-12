# JWT Sign

Signs a JSON object as a JSON Web Token using a provided secret key. Supports HMAC algorithms (HS256, HS384, HS512) and None.

- Input: `JSON`
- Output: `String`
- CLI: `rxchef run "JWT Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Private/Secret Key | `secret` | The secret key for HMAC signing |
| 2 | Signing algorithm | `HS256` | Algorithm: HS256, HS384, HS512, None |

