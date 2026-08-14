# Remove whitespace

## Overview

Optionally removes all spaces, carriage returns, line feeds, tabs and form feeds from the input data. This operation also supports the removal of full stops which are sometimes used to represent non-printable bytes in ASCII output.

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
| 1 | Spaces | `Boolean` | no | `true` | — | no | Remove spaces |
| 2 | Carriage returns (\\r) | `Boolean` | no | `true` | — | no | Remove carriage returns |
| 3 | Line feeds (\\n) | `Boolean` | no | `true` | — | no | Remove line feeds |
| 4 | Tabs | `Boolean` | no | `true` | — | no | Remove tabs |
| 5 | Form feeds (\\f) | `Boolean` | no | `true` | — | no | Remove form feeds |
| 6 | Full stops | `Boolean` | no | `false` | — | no | Remove full stops |

## How it works

Optionally removes all spaces, carriage returns, line feeds, tabs and form feeds from the input data. This operation also supports the removal of full stops which are sometimes used to represent non-printable bytes in ASCII output.

## Implementation

The implementation is in `src/operations/remove_whitespace.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Remove whitespace"
```

For file or binary input use `rxchef run "Remove whitespace" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Remove whitespace" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/remove_whitespace.rs

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
