# Derive PBKDF2 key

PBKDF2 is a password-based key derivation function. It is part of RSA Laboratories' Public-Key Cryptography Standards (PKCS) series, specifically PKCS #5 v2.0, also published as Internet Engineering Task Force's RFC 2898.<br><br>In many applications of cryptography, user security is ultimately dependent on a password, and because a password usually can't be used directly as a cryptographic key, some processing is required.<br><br>A salt provides a large set of keys for any given password, and an iteration count increases the cost of producing keys from a password, thereby also increasing the difficulty of attack.<br><br>If you leave the salt argument empty, a random salt will be generated.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Derive PBKDF2 key"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Passphrase | `<empty>` | The passphrase to derive the key from |
| 2 | Key size | `128` | The size of the derived key in bits |
| 3 | Iterations | `1` | The number of iterations to perform |
| 4 | Hashing function | `SHA256` | The hashing function to use |
| 5 | Salt | `<empty>` | The salt to use (if empty, a random one will be generated) |

