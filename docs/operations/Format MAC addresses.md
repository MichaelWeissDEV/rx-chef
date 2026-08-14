# Format MAC addresses

## Overview

Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas.

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
| 1 | Output case | `String` | no | `Both` | — | no | Both, Upper only, or Lower only |
| 2 | No delimiter | `Boolean` | no | `true` | — | no | Output format with no delimiter (e.g. AABBCCDDEEFF) |
| 3 | Dash delimiter | `Boolean` | no | `true` | — | no | Output format with dashes (e.g. AA-BB-CC-DD-EE-FF) |
| 4 | Colon delimiter | `Boolean` | no | `true` | — | no | Output format with colons (e.g. AA:BB:CC:DD:EE:FF) |
| 5 | Cisco style | `Boolean` | no | `false` | — | no | Output Cisco dot notation (e.g. AABB.CCDD.EEFF) |
| 6 | IPv6 interface ID | `Boolean` | no | `false` | — | no | Output as IPv6 interface identifier |

## How it works

Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas.

## Implementation

The implementation is in `src/operations/format_mac_addresses.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Format MAC addresses"
```

For file or binary input use `rxchef run "Format MAC addresses" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Format MAC addresses" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/format_mac_addresses.rs

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
