# Convert Image Format

Converts an image between different formats. Supported formats:<br><ul><li>Joint Photographic Experts Group (JPEG)</li><li>Portable Network Graphics (PNG)</li><li>Bitmap (BMP)</li><li>Tagged Image File Format (TIFF)</li></ul><br>Note: GIF files are supported for input, but cannot be outputted.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Convert Image Format"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Output Format | `JPEG` | The format to convert the image to |
| 2 | JPEG Quality | `80` | The quality of the JPEG output (1-100) |
| 3 | PNG Filter Type | `Auto` | The filter type to use for PNG output |
| 4 | PNG Deflate Level | `9` | The deflate level to use for PNG output (0-9) |

