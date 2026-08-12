# Generate HOTP

The HMAC-based One-Time Password algorithm (HOTP) is an algorithm that computes a one-time password from a shared secret key and an incrementing counter.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Generate HOTP"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Name | `<empty>` | Label for the HOTP |
| 2 | Code length | `6` | Number of digits in the code |
| 3 | Counter | `0` | Counter value |

