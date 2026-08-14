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
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

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

## How it works

Takes a single domain name and performs a DNS lookup using DNS over HTTPS.

By default, Cloudflare and Google DNS over HTTPS services are supported.

Can be used with any service that supports the GET parameters `name` and `type`.

## Implementation

The implementation is in `src/operations/dns_over_https.rs` and declares `String` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "DNS over HTTPS"
```

For file or binary input use `rxchef run "DNS over HTTPS" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "DNS over HTTPS" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/dns_over_https.rs

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
