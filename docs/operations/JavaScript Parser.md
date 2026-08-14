# JavaScript Parser

## Overview

Parses JavaScript and returns a SWC Abstract Syntax Tree as JSON. Optional source locations, byte ranges, tokens, comments, and recoverable parser errors can be included.

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
| 1 | Location info | `Boolean` | no | `false` | — | no | Include line and column location information |
| 2 | Range info | `Boolean` | no | `false` | — | no | Include range information |
| 3 | Include tokens array | `Boolean` | no | `false` | — | yes | Include tokens array |
| 4 | Include comments array | `Boolean` | no | `false` | — | no | Include comments array |
| 5 | Report errors and try to continue | `Boolean` | no | `false` | — | no | Report errors and try to continue |

## How it works

Parses JavaScript and returns a SWC Abstract Syntax Tree as JSON. Optional source locations, byte ranges, tokens, comments, and recoverable parser errors can be included.

## Implementation

The implementation is in `src/operations/java_script_parser.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "JavaScript Parser"
```

For file or binary input use `rxchef run "JavaScript Parser" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "JavaScript Parser" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/java_script_parser.rs

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
