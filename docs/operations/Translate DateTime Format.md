# Translate DateTime Format

## Overview

Parses a datetime string in one format and re-writes it in another. Uses strftime/strptime format strings (e.g. %d/%m/%Y %H:%M:%S). Timezone names are noted but conversion uses UTC unless a numeric offset is embedded in the format.

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
| 1 | Input format string | `String` | no | `%d/%m/%Y %H:%M:%S` | — | no | strftime format string for parsing input (e.g. %d/%m/%Y %H:%M:%S) |
| 2 | Input timezone | `String` | no | `UTC` | — | no | Timezone of the input datetime (informational; UTC assumed unless offset in format) |
| 3 | Output format string | `String` | no | `%A %d %B %Y %H:%M:%S` | — | no | strftime format string for the output (e.g. %A %d %B %Y %H:%M:%S %z) |
| 4 | Output timezone | `String` | no | `UTC` | — | no | Timezone for the output datetime (informational; UTC assumed unless offset in format) |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/translate_date_time_format.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Translate DateTime Format"
```

For file or binary input use `rxchef run "Translate DateTime Format" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Translate DateTime Format" to_base64
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
