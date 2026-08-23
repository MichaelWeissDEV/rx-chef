# Extract IP addresses

## Overview

Extracts all IPv4 and IPv6 addresses.

Warning: Given a string 1.2.3.4.5.6.7.8, this will match 1.2.3.4 and 5.6.7.8 so always check the original input!

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Exact` |
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
| 1 | IPv4 | `Boolean` | no | `true` | — | no | Include IPv4 addresses |
| 2 | IPv6 | `Boolean` | no | `false` | — | no | Include IPv6 addresses |
| 3 | Remove local IPv4 addresses | `Boolean` | no | `false` | — | no | Exclude local/private IPv4 addresses |
| 4 | Display total | `Boolean` | no | `false` | — | no | Display the total number of addresses found |
| 5 | Sort | `Boolean` | no | `false` | — | no | Sort the results |
| 6 | Unique | `Boolean` | no | `false` | — | no | Remove duplicate results |

## Implementation

The implementation is in `src/operations/extract_ip_addresses.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Extract IP addresses" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Exact`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/extract_ip_addresses.rs
- tests/tests/negative_contracts.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
