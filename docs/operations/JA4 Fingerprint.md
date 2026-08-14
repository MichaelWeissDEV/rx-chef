# JA4 Fingerprint

## Overview

Generates a JA4 fingerprint to help identify TLS clients based on hashing together values from the Client Hello.<br><br>Input: A hex stream of the TLS or QUIC Client Hello packet application layer.

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
| 1 | Input format | `String` | no | `Hex` | — | no | Input format |
| 2 | Output format | `String` | no | `JA4` | — | no | Output format |

## How it works

Generates a JA4 fingerprint to help identify TLS clients based on hashing together values from the Client Hello.<br><br>Input: A hex stream of the TLS or QUIC Client Hello packet application layer.

## Implementation

The implementation is in `src/operations/ja4_fingerprint.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "JA4 Fingerprint"
```

For file or binary input use `rxchef run "JA4 Fingerprint" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "JA4 Fingerprint" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/ja4_fingerprint.rs

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
