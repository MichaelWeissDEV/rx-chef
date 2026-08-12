# SHA2

The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. The message digest algorithm for SHA256 variants consists, by default, of 64 rounds, and for SHA512 variants, it is, by default, 160.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SHA2"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `256` | Output size (224, 256, 384, 512, 512/256, 512/224) |
| 2 | Rounds (SHA-256) | `64` | Number of rounds for 256/224 (minimum 16) |
| 3 | Rounds (SHA-512) | `160` | Number of rounds for 512/384/224/256 (minimum 32) |

