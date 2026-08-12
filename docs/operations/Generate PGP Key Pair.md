# Generate PGP Key Pair

Generates a new public/private PGP key pair. Supports RSA (1024/2048/4096) and ECC (256/384/521) key types. Arguments: key type, optional password, optional name, optional email.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Generate PGP Key Pair"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key type | `RSA-2048` | Key type and size: RSA-1024, RSA-2048, RSA-4096, ECC-256, ECC-384, ECC-521 |
| 2 | Password (optional) | `<empty>` | Passphrase to protect the private key |
| 3 | Name (optional) | `<empty>` | User name for the key identity |
| 4 | Email (optional) | `<empty>` | User email for the key identity |

