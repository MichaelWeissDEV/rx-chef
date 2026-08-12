# Generate QR Code

Generates a Quick Response (QR) code from the input text.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "Generate QR Code"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Image Format | `PNG` | Format of the QR code image |
| 2 | Module size (px) | `5` | Size of each module in pixels |
| 3 | Margin (num modules) | `4` | Margin around the QR code in modules |
| 4 | Error correction | `Medium` | Error correction level |

