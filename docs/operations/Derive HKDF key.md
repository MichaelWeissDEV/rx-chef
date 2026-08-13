# Derive HKDF key

## Overview

A simple Hashed Message Authenticaton Code (HMAC)-based key derivation function (HKDF), defined in RFC5869.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Salt | `String` | no | `<empty>` | — | no | The salt to use |
| 2 | Info | `String` | no | `<empty>` | — | no | The info to use |
| 3 | Hashing function | `String` | no | `SHA256` | — | no | The hashing function to use (SHA1, SHA256, SHA384, SHA512) |
| 4 | Extract mode | `String` | no | `with salt` | — | no | The extract mode (with salt, no salt, skip) |
| 5 | L (number of output octets) | `Integer` | no | `16` | — | no | The number of output octets |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/derive_hkdf_key.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Derive HKDF key"
```

For file or binary input use `rxchef run "Derive HKDF key" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Derive HKDF key" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
