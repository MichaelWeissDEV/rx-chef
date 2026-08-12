# Scrypt

scrypt is a password-based key derivation function (PBKDF) created by Colin Percival. The algorithm was specifically designed to make it costly to perform large-scale custom hardware attacks by requiring large amounts of memory. Enter the password in the input to generate its hash.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Scrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Salt | `<empty>` | Salt |
| 2 | Iterations (N) | `16384` | Iterations (N). Must be a power of 2. |
| 3 | Memory factor (r) | `8` | Memory factor (r) |
| 4 | Parallelization factor (p) | `1` | Parallelization factor (p) |
| 5 | Key length | `64` | Key length |

