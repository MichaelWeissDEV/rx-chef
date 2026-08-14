# Argon2

## Overview

Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Salt | `Bytes` | no | `somesalt` | — | no | Salt value |
| 2 | Iterations | `UnsignedInteger` | no | `3` | — | no | Number of iterations |
| 3 | Memory (KiB) | `Integer` | no | `4096` | — | no | Memory usage in KiB |
| 4 | Parallelism | `Integer` | no | `1` | — | no | Degree of parallelism |
| 5 | Hash length (bytes) | `UnsignedInteger` | no | `32` | — | no | Length of the hash in bytes |
| 6 | Type | `Enum` | no | `Argon2i` | Argon2i, Argon2d, Argon2id | no | Argon2 type (Argon2i, Argon2d, Argon2id) |
| 7 | Output format | `Enum` | no | `Encoded hash` | Encoded hash, Hex hash, Raw hash | no | Output format (Encoded hash, Hex hash, Raw hash) |

## How it works

Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.

## Implementation

The implementation is in `src/operations/argon2.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Argon2"
```

For file or binary input use `rxchef run "Argon2" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Argon2" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/argon2.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
