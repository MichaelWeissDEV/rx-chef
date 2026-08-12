# Derive EVP key

This operation performs a password-based key derivation function (PBKDF) used extensively in OpenSSL. In many applications of cryptography, user security is ultimately dependent on a password, and because a password usually can't be used directly as a cryptographic key, some processing is required.<br><br>A salt provides a large set of keys for any given password, and an iteration count increases the cost of producing keys from a password, thereby also increasing the difficulty of attack.<br><br>If you leave the salt argument empty, a random salt will be generated.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Derive EVP key"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Passphrase | `<empty>` | The passphrase to derive the key from. |
| 2 | Key size | `128` | The length of the key to generate in bits. |
| 3 | Iterations | `1` | The number of times the hash function is applied. |
| 4 | Hashing function | `MD5` | The hash function to use. |
| 5 | Salt | `<empty>` | The salt to use. If empty, a random salt will be generated. |

