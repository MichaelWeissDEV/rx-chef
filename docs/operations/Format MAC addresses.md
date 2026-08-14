# Format MAC addresses

## Overview

Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Required` |
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

## Implementation

The implementation is in `src/operations/format_mac_addresses.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Format MAC addresses" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/format_mac_addresses.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
