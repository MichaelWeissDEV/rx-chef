# Windows Filetime to UNIX Timestamp

## Overview

Converts a Windows Filetime value to a UNIX timestamp.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds.

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
| 1 | Output units | `String` | no | `Seconds (s)` | — | no | Units for the output timestamp |
| 2 | Input format | `String` | no | `Decimal` | — | no | Format of the input filetime |

## How it works

Converts a Windows Filetime value to a UNIX timestamp.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds.

## Implementation

The implementation is in `src/operations/windows_filetime_to_unix_timestamp.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Windows Filetime to UNIX Timestamp"
```

For file or binary input use `rxchef run "Windows Filetime to UNIX Timestamp" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Windows Filetime to UNIX Timestamp" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/windows_filetime_to_unix_timestamp.rs

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
