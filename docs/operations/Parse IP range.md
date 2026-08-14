# Parse IP range

## Overview

Given a CIDR range (e.g. 10.0.0.0/24), a hyphenated range (e.g. 10.0.0.0 - 10.0.1.0), or a single IP address, this operation provides network information and enumerates all IP addresses in the range. IPv6 is supported but not enumerated.

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
| 1 | Include network info | `Boolean` | no | `true` | — | no | Show network/mask/host information |
| 2 | Enumerate IP addresses | `Boolean` | no | `true` | — | no | List every IP in the range |
| 3 | Allow large queries | `Boolean` | no | `false` | — | no | Allow ranges larger than 65536 |

## How it works

Given a CIDR range (e.g. 10.0.0.0/24), a hyphenated range (e.g. 10.0.0.0 - 10.0.1.0), or a single IP address, this operation provides network information and enumerates all IP addresses in the range. IPv6 is supported but not enumerated.

## Implementation

The implementation is in `src/operations/parse_ip_range.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Parse IP range"
```

For file or binary input use `rxchef run "Parse IP range" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Parse IP range" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/parse_ip_range.rs

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
