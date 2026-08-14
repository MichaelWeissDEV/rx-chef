# ROT47 Brute Force

## Overview

Try all meaningful amounts for ROT47. Optionally you can enter your known plaintext (crib) to filter the result.

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
| 1 | Sample length | `UnsignedInteger` | no | `100` | — | no | Number of bytes to sample from input |
| 2 | Sample offset | `Integer` | no | `0` | — | no | Byte offset to start sampling |
| 3 | Print amount | `Boolean` | no | `true` | — | no | Prefix each result with its rotation amount (true/false) |
| 4 | Crib (known plaintext string) | `String` | no | `<empty>` | — | no | Filter results to those containing this string |

## How it works

Try all meaningful amounts for ROT47. Optionally you can enter your known plaintext (crib) to filter the result.

## Implementation

The implementation is in `src/operations/rot47_brute_force.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "ROT47 Brute Force"
```

For file or binary input use `rxchef run "ROT47 Brute Force" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "ROT47 Brute Force" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/rot47_brute_force.rs

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
