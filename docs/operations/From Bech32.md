# From Bech32

Bech32 is an encoding scheme primarily used for Bitcoin SegWit addresses (BIP-0173). It uses a 32-character alphabet that excludes easily confused characters (1, b, i, o) and includes a checksum for error detection.

Bech32m (BIP-0350) is an updated version used for Bitcoin Taproot addresses.

Auto-detect will attempt Bech32 first, then Bech32m if the checksum fails.

Output format options allow you to see the Human-Readable Part (HRP) along with the decoded data.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "From Bech32"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Encoding | `Auto-detect` | The Bech32 encoding variant |
| 2 | Output Format | `Raw` | The format of the output |

