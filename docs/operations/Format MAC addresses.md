# Format MAC addresses

## Overview

Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
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

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/format_mac_addresses.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

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

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
