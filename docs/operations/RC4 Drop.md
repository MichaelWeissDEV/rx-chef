# RC4 Drop

It was discovered that the first few bytes of the RC4 keystream are strongly non-random and leak information about the key. We can defend against this attack by discarding the initial portion of the keystream. This modified algorithm is traditionally called RC4-drop.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "RC4 Drop"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Passphrase/key as UTF-8 string or hex (prefix 0x for hex) |
| 2 | Input format | `Raw` | Input encoding: Raw or Hex |
| 3 | Output format | `Hex` | Output encoding: Raw or Hex |
| 4 | Number of dwords to drop | `192` | Number of 4-byte dwords to discard from keystream start (default: 192) |

