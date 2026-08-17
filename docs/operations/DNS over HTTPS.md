# DNS over HTTPS

## Overview

Takes a single domain name and performs a DNS lookup using DNS over HTTPS.

By default, Cloudflare and Google DNS over HTTPS services are supported.

Can be used with any service that supports the GET parameters `name` and `type`.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Required` |
| Features | none |
| Side effects | `[Network]` |
| Deterministic | false |

## Input

Declared input type: `String`.

## Output

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Resolver | `String` | no | `https://cloudflare-dns.com/dns-query` | — | no | The DNS over HTTPS resolver URL (e.g., Google or Cloudflare). |
| 2 | Request Type | `String` | no | `A` | — | no | The type of DNS request (A, AAAA, TXT, etc.). |
| 3 | Answer Data Only | `Boolean` | no | `false` | — | no | If true, only returns the Answer section data values. |
| 4 | Disable DNSSEC validation | `Boolean` | no | `false` | — | no | Disable DNSSEC validation (Checking Disabled). |

## Implementation

The implementation is in `src/operations/dns_over_https.rs` and declares `String` input and `JSON` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "DNS over HTTPS" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `JSON` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[Network]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/dns_over_https.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
