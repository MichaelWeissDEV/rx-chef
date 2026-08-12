# Argon2

Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Argon2"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Salt | `somesalt` | Salt value |
| 2 | Iterations | `3` | Number of iterations |
| 3 | Memory (KiB) | `4096` | Memory usage in KiB |
| 4 | Parallelism | `1` | Degree of parallelism |
| 5 | Hash length (bytes) | `32` | Length of the hash in bytes |
| 6 | Type | `Argon2i` | Argon2 type (Argon2i, Argon2d, Argon2id) |
| 7 | Output format | `Encoded hash` | Output format (Encoded hash, Hex hash, Raw hash) |

