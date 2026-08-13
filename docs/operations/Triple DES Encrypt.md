# Triple DES Encrypt

## Overview

Encrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Raw or Hex.

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

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key | `String` | no | `<empty>` | — | no | Encryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64 |
| 2 | Key encoding | `String` | no | `Hex` | — | no | Encoding of the key: Hex, UTF8, Latin1, Base64 |
| 3 | IV | `String` | no | `<empty>` | — | no | Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64 |
| 4 | IV encoding | `String` | no | `Hex` | — | no | Encoding of the IV: Hex, UTF8, Latin1, Base64 |
| 5 | Mode | `String` | no | `CBC` | — | no | Cipher mode: CBC, ECB |
| 6 | Input | `String` | no | `Raw` | — | no | Input encoding: Raw, Hex |
| 7 | Output | `String` | no | `Hex` | — | no | Output encoding: Hex, Raw |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/triple_des_encrypt.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Triple DES Encrypt"
```

For file or binary input use `rxchef run "Triple DES Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Triple DES Encrypt" to_base64
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
