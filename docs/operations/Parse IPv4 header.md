# Parse IPv4 header

## Overview

Parses an IPv4 packet header and displays each field in a readable format including version, IHL, DSCP, ECN, total length, identification, flags, fragment offset, TTL, protocol, checksum, source and destination IP addresses, and options if present.

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
| 1 | Input format | `String` | no | `Hex` | — | no | Hex or Raw |
| 2 | Output format | `String` | no | `Table` | — | no | Table, Data (hex), or Data (raw) |

## How it works

Parses an IPv4 packet header and displays each field in a readable format including version, IHL, DSCP, ECN, total length, identification, flags, fragment offset, TTL, protocol, checksum, source and destination IP addresses, and options if present.

## Implementation

The implementation is in `src/operations/parse_ipv4_header.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Parse IPv4 header"
```

For file or binary input use `rxchef run "Parse IPv4 header" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Parse IPv4 header" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/parse_ipv4_header.rs

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
