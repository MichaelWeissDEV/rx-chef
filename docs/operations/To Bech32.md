# To Bech32

Bech32 is an encoding scheme primarily used for Bitcoin SegWit addresses (BIP-0173). It uses a 32-character alphabet that excludes easily confused characters (1, b, i, o) and includes a checksum for error detection.<br><br>Bech32m (BIP-0350) is an updated version that fixes a weakness in the original Bech32 checksum and is used for Bitcoin Taproot addresses.<br><br>The Human-Readable Part (HRP) identifies the network or purpose (e.g., 'bc' for Bitcoin mainnet, 'tb' for testnet, 'age' for AGE encryption keys).<br><br>Maximum output length is 90 characters as per specification.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "To Bech32"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Human-Readable Part (HRP) | `bc` | Human-Readable Part (HRP) |
| 2 | Encoding | `Bech32` | Encoding |
| 3 | Input Format | `Raw bytes` | Input Format |
| 4 | Mode | `Generic` | Mode |
| 5 | Witness Version | `0` | Witness Version (0-16) |

