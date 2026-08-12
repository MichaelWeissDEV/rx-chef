# Parse X.509 certificate

X.509 is an ITU-T standard for a public key infrastructure (PKI) and Privilege Management Infrastructure (PMI). It is commonly involved with SSL/TLS security.<br><br>This operation displays the contents of a certificate in a human readable format, similar to the openssl command line tool.<br><br>Tags: X509, server hello, handshake

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse X.509 certificate"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `PEM` | Input format of the certificate |

