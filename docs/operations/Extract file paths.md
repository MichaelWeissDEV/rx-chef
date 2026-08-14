# Extract file paths

## Overview

Extracts anything that looks like a Windows or UNIX file path.

Note that if UNIX is selected, there will likely be a lot of false positives.

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
| 1 | Windows | `Boolean` | no | `true` | — | no | Include Windows file paths |
| 2 | UNIX | `Boolean` | no | `true` | — | no | Include UNIX file paths |
| 3 | Display total | `Boolean` | no | `false` | — | no | Display the total number of paths found |
| 4 | Sort | `Boolean` | no | `false` | — | no | Sort the results |
| 5 | Unique | `Boolean` | no | `false` | — | no | Remove duplicate results |

## How it works

Extracts anything that looks like a Windows or UNIX file path.

Note that if UNIX is selected, there will likely be a lot of false positives.

## Implementation

The implementation is in `src/operations/extract_file_paths.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Extract file paths"
```

For file or binary input use `rxchef run "Extract file paths" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Extract file paths" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/extract_file_paths.rs

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
