# Bcrypt

bcrypt is a password hashing function designed by Niels Provos and David Mazires, based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating a salt to protect against rainbow table attacks, bcrypt is an adaptive function: over time, the iteration count (rounds) can be increased to make it slower.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Bcrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Rounds | `10` | Number of rounds (10-31, default 10) |

