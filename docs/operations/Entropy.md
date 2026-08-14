# Entropy

## Overview

Shannon Entropy, in the context of information theory, is a measure of the rate at which information is produced by a source of data. 8 is the maximum, representing highly unstructured, random data. English language text usually falls somewhere between 3.5 and 5. Properly encrypted or compressed data should have an entropy of over 7.5.

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
| 1 | Chunk size | `UnsignedInteger` | no | `0` | — | no | Size of each chunk for scanning entropy. 0 means calculate for whole input. |

## How it works

Shannon Entropy, in the context of information theory, is a measure of the rate at which information is produced by a source of data. 8 is the maximum, representing highly unstructured, random data. English language text usually falls somewhere between 3.5 and 5. Properly encrypted or compressed data should have an entropy of over 7.5.

## Implementation

The implementation is in `src/operations/entropy.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Entropy"
```

For file or binary input use `rxchef run "Entropy" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Entropy" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/entropy.rs

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
