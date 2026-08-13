# RC6 Decrypt

## Overview

RC6 is a symmetric key block cipher derived from RC5. It was designed by Ron Rivest, Matt Robshaw, Ray Sidney, and Yiqun Lisa Yin to meet the requirements of the AES competition, and was one of the five finalists.<br><br>RC6 is parameterised as RC6-w/r/b where w is word size in bits (any multiple of 8 from 8-256), r is the number of rounds (1-255), and b is the key length in bytes. The standard AES submission uses w=32, r=20. Common word sizes: 8, 16, 32 (standard), 64, 128.<br><br><b>IV:</b> The Initialisation Vector should be 4*w/8 bytes (e.g. 16 bytes for w=32). If not entered, it will default to null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, the PKCS#7 padding scheme is used.

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
| 1 | Key | `String` | no | `<empty>` | — | no | Key |
| 2 | IV | `String` | no | `<empty>` | — | no | IV |
| 3 | Mode | `String` | no | `CBC` | — | no | Mode |
| 4 | Input | `String` | no | `Hex` | — | no | Input format |
| 5 | Output | `String` | no | `Raw` | — | no | Output format |
| 6 | Padding | `String` | no | `PKCS5` | — | no | Padding scheme |
| 7 | Word Size | `Integer` | no | `32` | — | no | Word size in bits (8-256) |
| 8 | Rounds | `Integer` | no | `20` | — | no | Number of rounds (1-255) |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/rc6_decrypt.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "RC6 Decrypt"
```

For file or binary input use `rxchef run "RC6 Decrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "RC6 Decrypt" to_base64
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
