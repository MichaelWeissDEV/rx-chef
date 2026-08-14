# Levenshtein Distance

## Overview

Levenshtein Distance (also known as Edit Distance) is a string metric to measure a difference between two strings that counts operations (insertions, deletions, and substitutions) on single character that are required to change one string to another.

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
| 1 | Sample delimiter | `String` | no | `\\n` | — | no | Delimiter separating the two input samples |
| 2 | Insertion cost | `Integer` | no | `1` | — | no | Cost of inserting a character |
| 3 | Deletion cost | `Integer` | no | `1` | — | no | Cost of deleting a character |
| 4 | Substitution cost | `Integer` | no | `1` | — | no | Cost of substituting a character |

## How it works

Levenshtein Distance (also known as Edit Distance) is a string metric to measure a difference between two strings that counts operations (insertions, deletions, and substitutions) on single character that are required to change one string to another.

## Implementation

The implementation is in `src/operations/levenshtein_distance.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Levenshtein Distance"
```

For file or binary input use `rxchef run "Levenshtein Distance" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Levenshtein Distance" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/levenshtein_distance.rs

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
