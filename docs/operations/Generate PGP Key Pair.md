# Generate PGP Key Pair

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Generates a new public/private PGP key pair. Supports RSA (1024/2048/4096) and ECC (256/384/521) key types. Arguments: key type, optional password, optional name, optional email.

## Status

| Field | Value |
|---|---|
| Implementation | `FeatureGated` |
| Parity | `Unknown` |
| Availability | unavailable in this build |
| Features | pgp |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key type | `String` | no | `RSA-2048` | — | no | Key type and size: RSA-2048, RSA-4096, ECC-256, ECC-384, ECC-521 (RSA-1024 is rejected as insecure) |
| 2 | Password (optional) | `String` | no | `<empty>` | — | yes | Passphrase to protect the private key |
| 3 | Name (optional) | `String` | no | `<empty>` | — | no | User name for the key identity |
| 4 | Email (optional) | `String` | no | `<empty>` | — | no | User email for the key identity |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/generate_pgp_key_pair.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Generate PGP Key Pair"
```

For file or binary input use `rxchef run "Generate PGP Key Pair" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Generate PGP Key Pair" to_base64
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
