# Magic

## Overview

The Magic operation attempts to detect various properties of the input data and suggests which operations could help to make more sense of it.

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

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Depth | `UnsignedInteger` | no | `3` | — | no | Maximum number of levels of recursion |
| 2 | Intensive mode | `Boolean` | no | `false` | — | no | Brute-force XOR, bit rotates, etc. |
| 3 | Extensive language support | `Boolean` | no | `false` | — | no | Compare byte frequencies to a large number of languages |
| 4 | Crib (known plaintext string or regex) | `Regex` | no | `<empty>` | — | no | Filter results by matching this string or regex |

## How it works

The Magic operation attempts to detect various properties of the input data and suggests which operations could help to make more sense of it.

## Implementation

The implementation is in `src/operations/magic.rs` and declares `Bytes` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Magic"
```

For file or binary input use `rxchef run "Magic" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Magic" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/magic.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Benchmark evidence:
- representative release benchmark

See [benchmark results](../performance/results.md) for measured environment and statistics.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
