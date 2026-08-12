# Generate TOTP

The Time-based One-Time Password algorithm (TOTP) is an algorithm that computes a one-time password from a shared secret key and the current time. It has been adopted as Internet Engineering Task Force standard RFC 6238, is the cornerstone of Initiative For Open Authentication (OAUTH), and is used in a number of two-factor authentication systems. A TOTP is an HOTP where the counter is the current time.

Enter the secret as the input or leave it blank for a random secret to be generated. T0 and T1 are in seconds.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Generate TOTP"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Name | `<empty>` | The name of the account |
| 2 | Code length | `6` | The number of digits in the generated code |
| 3 | Epoch offset (T0) | `0` | The epoch offset in seconds |
| 4 | Interval (T1) | `30` | The time interval in seconds |

