# Swap endianness

## Overview

Switches the data from big-endian to little-endian or vice-versa. Data can be read in as hexadecimal or raw bytes. It will be returned in the same format as it is entered.

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
| 1 | Data format | `String` | no | `Raw` | — | no | Input/output format: Hex or Raw |
| 2 | Word length (bytes) | `UnsignedInteger` | no | `4` | — | no | Number of bytes per word |
| 3 | Pad incomplete words | `Boolean` | no | `true` | — | no | If true, pad incomplete words with zero bytes |

## How it works

Switches the data from big-endian to little-endian or vice-versa. Data can be read in as hexadecimal or raw bytes. It will be returned in the same format as it is entered.

## Implementation

The implementation is in `src/operations/swap_endianness.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Swap endianness"
```

For file or binary input use `rxchef run "Swap endianness" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Swap endianness" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/swap_endianness.rs

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
