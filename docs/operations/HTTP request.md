# HTTP request

## Overview

Makes an HTTP request and returns the response.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Optional` |
| Features | none |
| Side effects | `[Network]` |
| Deterministic | false |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Method | `String` | no | `GET` | — | no | HTTP method |
| 2 | URL | `Url` | yes | `<empty>` | — | no | The URL to request |
| 3 | Headers | `String` | no | `<empty>` | — | no | Request headers (Key: Value) |
| 4 | Mode | `String` | no | `Cross-Origin Resource Sharing` | — | no | CORS mode (ignored in Rust) |
| 5 | Show response metadata | `Boolean` | no | `false` | — | no | Include status and headers in output |

## Implementation

The implementation is in `src/operations/http_request.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

Input is optional. Omit all input selectors to use the operation's no-input behavior, or provide text, a file, or stdin explicitly.

```console
rxchef run "HTTP request" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[Network]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/http_request.rs

Known-answer tests:
- tests/tests/operations/http_request.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
