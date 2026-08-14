# Defang URL

## Overview

Takes a Universal Resource Locator (URL) and 'Defangs' it; meaning the URL becomes invalid, neutralising the risk of accidentally clicking on a malicious link.

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
| 1 | Escape dots | `Boolean` | no | `true` | — | no | Escape dots in the URL |
| 2 | Escape http | `Boolean` | no | `true` | — | no | Escape http/https in the URL |
| 3 | Escape :// | `Boolean` | no | `true` | — | no | Escape :// in the URL |
| 4 | Process | `String` | no | `Valid domains and full URLs` | — | no | Process option |

## How it works

Takes a Universal Resource Locator (URL) and 'Defangs' it; meaning the URL becomes invalid, neutralising the risk of accidentally clicking on a malicious link.

## Implementation

The implementation is in `src/operations/defang_url.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Defang URL"
```

For file or binary input use `rxchef run "Defang URL" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Defang URL" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/defang_url.rs

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
