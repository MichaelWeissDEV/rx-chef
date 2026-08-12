# RC4

RC4 (also known as ARC4) is a widely-used stream cipher designed by Ron Rivest. It is used in popular protocols such as SSL and WEP. Although remarkable for its simplicity and speed, the algorithm's history doesn't inspire confidence in its security.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "RC4"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Passphrase/key as UTF-8 string or hex (prefix 0x for hex) |
| 2 | Input format | `Raw` | Input encoding: Raw or Hex |
| 3 | Output format | `Hex` | Output encoding: Raw or Hex |

