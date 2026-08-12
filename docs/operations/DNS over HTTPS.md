# DNS over HTTPS

Takes a single domain name and performs a DNS lookup using DNS over HTTPS.

By default, Cloudflare and Google DNS over HTTPS services are supported.

Can be used with any service that supports the GET parameters `name` and `type`.

- Input: `String`
- Output: `JSON`
- CLI: `rxchef run "DNS over HTTPS"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Resolver | `https://cloudflare-dns.com/dns-query` | The DNS over HTTPS resolver URL (e.g., Google or Cloudflare). |
| 2 | Request Type | `A` | The type of DNS request (A, AAAA, TXT, etc.). |
| 3 | Answer Data Only | `false` | If true, only returns the Answer section data values. |
| 4 | Disable DNSSEC validation | `false` | Disable DNSSEC validation (Checking Disabled). |

