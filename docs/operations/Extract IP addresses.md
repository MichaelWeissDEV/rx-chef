# Extract IP addresses

## Overview

Extracts all IPv4 and IPv6 addresses.

Warning: Given a string 1.2.3.4.5.6.7.8, this will match 1.2.3.4 and 5.6.7.8 so always check the original input!

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
| 1 | IPv4 | `Boolean` | no | `true` | — | no | Include IPv4 addresses |
| 2 | IPv6 | `Boolean` | no | `false` | — | no | Include IPv6 addresses |
| 3 | Remove local IPv4 addresses | `Boolean` | no | `false` | — | no | Exclude local/private IPv4 addresses |
| 4 | Display total | `Boolean` | no | `false` | — | no | Display the total number of addresses found |
| 5 | Sort | `Boolean` | no | `false` | — | no | Sort the results |
| 6 | Unique | `Boolean` | no | `false` | — | no | Remove duplicate results |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/extract_ip_addresses.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Extract IP addresses"
```

For file or binary input use `rxchef run "Extract IP addresses" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Extract IP addresses" to_base64
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
