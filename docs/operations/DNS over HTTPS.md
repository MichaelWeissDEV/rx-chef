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
| Availability | available |
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

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/dns_over_https.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

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

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
