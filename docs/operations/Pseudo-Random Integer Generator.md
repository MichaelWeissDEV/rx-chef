# Pseudo-Random Integer Generator

## Overview

A cryptographically-secure pseudo-random number generator (PRNG). Generates random integers within a specified range. The supported range of integers is from -(2^53 - 1) to (2^53 - 1).

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
| 1 | Number of Integers | `Integer` | no | `1` | — | no | How many integers to generate |
| 2 | Min Value | `Integer` | no | `0` | — | no | Minimum value (inclusive) |
| 3 | Max Value | `Integer` | no | `99` | — | no | Maximum value (inclusive) |
| 4 | Delimiter | `String` | no | `Space` | — | no | Delimiter between integers |
| 5 | Output | `String` | no | `Decimal` | — | no | Output format (Raw, Hex, Decimal) |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/pseudo_random_integer_generator.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Pseudo-Random Integer Generator"
```

For file or binary input use `rxchef run "Pseudo-Random Integer Generator" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Pseudo-Random Integer Generator" to_base64
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
