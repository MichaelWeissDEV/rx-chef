# Conditional Jump

## Overview

Conditionally jump forwards or backwards to a Label when the current data matches a regular expression. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends.

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
| 1 | Match (regex) | `Regex` | no | `<empty>` | — | no | The regular expression to match against the data |
| 2 | Invert match | `Boolean` | no | `false` | — | no | If true, jump when the regex does NOT match |
| 3 | Label name | `String` | no | `<empty>` | — | no | The name of the label to jump to |
| 4 | Maximum jumps (if jumping backwards) | `Integer` | no | `10` | — | no | The maximum number of times to jump backwards to prevent infinite loops |

## How it works

Conditionally jump forwards or backwards to a Label when the current data matches a regular expression. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends.

## Implementation

The implementation is in `src/operations/conditional_jump.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Conditional Jump"
```

For file or binary input use `rxchef run "Conditional Jump" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Conditional Jump" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/conditional_jump.rs

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
