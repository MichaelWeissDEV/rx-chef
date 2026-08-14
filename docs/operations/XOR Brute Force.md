# XOR Brute Force

## Overview

Enumerate all possible XOR solutions. Optionally enter a string that you expect to find in the plaintext to filter results (crib).

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

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key length | `UnsignedInteger` | no | `1` | — | no | Length of the XOR key in bytes (1..=2 recommended) |
| 2 | Sample length | `UnsignedInteger` | no | `100` | — | no | Number of bytes of input to process |
| 3 | Sample offset | `Integer` | no | `0` | — | no | Byte offset to start sampling from |
| 4 | Scheme | `String` | no | `Standard` | — | no | Standard, Input differential, or Output differential |
| 5 | Null preserving | `Boolean` | no | `false` | — | no | Do not XOR null bytes or bytes equal to the key |
| 6 | Print key | `Boolean` | no | `true` | — | no | Prefix each result with the key used |
| 7 | Output as hex | `Boolean` | no | `false` | — | no | Output results as hex instead of text |
| 8 | Crib (known plaintext string) | `String` | no | `<empty>` | — | no | Filter results to those containing this string |

## How it works

Enumerate all possible XOR solutions. Optionally enter a string that you expect to find in the plaintext to filter results (crib).

## Implementation

The implementation is in `src/operations/xor_brute_force.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "XOR Brute Force"
```

For file or binary input use `rxchef run "XOR Brute Force" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "XOR Brute Force" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/xor_brute_force.rs

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
