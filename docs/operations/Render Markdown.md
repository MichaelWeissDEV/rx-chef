# Render Markdown

## Overview

Renders Markdown as safe HTML. Raw HTML is escaped, URLs can be linked automatically, fenced code blocks can be syntax-highlighted, and links can open in a new tab.

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

Declared output type: `HTML`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Autoconvert URLs to links | `Boolean` | no | `false` | — | no | Autoconvert URLs to links |
| 2 | Enable syntax highlighting | `Boolean` | no | `true` | — | no | Highlight strings, numbers, comments, and common language keywords in fenced code blocks |
| 3 | Open links in new tab. | `Boolean` | no | `false` | — | no | Adds target="_blank" to links. |

## How it works

Renders Markdown as safe HTML. Raw HTML is escaped, URLs can be linked automatically, fenced code blocks can be syntax-highlighted, and links can open in a new tab.

## Implementation

The implementation is in `src/operations/render_markdown.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Render Markdown"
```

For file or binary input use `rxchef run "Render Markdown" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Render Markdown" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/render_markdown.rs

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
