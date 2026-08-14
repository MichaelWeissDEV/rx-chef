# Find / Replace

## Overview

Replaces all occurrences of the first string with the second. Supports regex, simple string, and extended string modes.

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
| 1 | Find | `String` | no | `<empty>` | — | no | The string or regex to find |
| 2 | Find type | `Enum` | no | `Simple string` | Simple string, Extended, Regex | no | Regex, Extended (\\n, \\t, \\x...), or Simple string |
| 3 | Replace | `String` | no | `<empty>` | — | no | The replacement string |
| 4 | Global match | `Boolean` | no | `true` | — | no | Replace all occurrences |
| 5 | Case insensitive | `Boolean` | no | `false` | — | no | Ignore case when matching |
| 6 | Multiline matching | `Boolean` | no | `true` | — | no | ^ and $ match start/end of lines |
| 7 | Dot matches all | `Boolean` | no | `false` | — | no | Dot also matches newline |

## How it works

Replaces all occurrences of the first string with the second. Supports regex, simple string, and extended string modes.

## Implementation

The implementation is in `src/operations/find_replace.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Find / Replace"
```

For file or binary input use `rxchef run "Find / Replace" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Find / Replace" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/find_replace.rs

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
