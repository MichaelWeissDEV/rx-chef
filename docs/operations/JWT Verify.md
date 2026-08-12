# JWT Verify

Verifies that a JSON Web Token is valid and has been signed with the provided secret key. Supports HS256, HS384, HS512, and None.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "JWT Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Public/Secret Key | `secret` | The secret key used to verify the HMAC signature |

