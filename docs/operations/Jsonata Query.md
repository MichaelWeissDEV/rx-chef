# Jsonata Query

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Features | jsonata |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `JSON`.

## Output

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Query | `String` | no | `.` | — | no | The jaq query to run |

## How it works

Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata

## Implementation

The implementation is in `src/operations/jsonata.rs` and declares `JSON` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Jsonata Query"
```

For file or binary input use `rxchef run "Jsonata Query" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Jsonata Query" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/jsonata.rs

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
