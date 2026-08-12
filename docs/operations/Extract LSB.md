# Extract LSB

Extracts the Least Significant Bit data from each pixel in an image. This is a common way to hide data in Steganography.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Extract LSB"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Colour Pattern #1 | `R` | Colour to extract from |
| 2 | Colour Pattern #2 | `<empty>` | Colour to extract from |
| 3 | Colour Pattern #3 | `<empty>` | Colour to extract from |
| 4 | Colour Pattern #4 | `<empty>` | Colour to extract from |
| 5 | Pixel Order | `Row` | Order to process pixels |
| 6 | Bit | `0` | Bit to extract (0-7) |

