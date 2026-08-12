# Parse QR Code

Reads an image file and attempts to detect and read a Quick Response (QR) code from the image.<br><br><u>Normalise Image</u><br>Attempts to normalise the image before parsing it to improve detection of a QR code.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Parse QR Code"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Normalise image | `false` | Attempts to normalise the image before parsing it to improve detection of a QR code. |

