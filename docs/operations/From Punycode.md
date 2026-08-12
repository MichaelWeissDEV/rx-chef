# From Punycode

Punycode is a way to represent Unicode with the limited character subset of ASCII supported by the Domain Name System. e.g. 'mnchen-3ya' decodes to 'muenchen'.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "From Punycode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Internationalised domain name | `false` | Treat input as a full IDN domain name (xn-- labels) |

