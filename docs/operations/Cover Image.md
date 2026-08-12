# Cover Image

Scales the image to the given width and height, keeping the aspect ratio. The image may be clipped.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Cover Image"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Width | `100` | The width of the covered image |
| 2 | Height | `100` | The height of the covered image |
| 3 | Horizontal align | `Center` | The horizontal alignment of the image within the cover area |
| 4 | Vertical align | `Middle` | The vertical alignment of the image within the cover area |
| 5 | Resizing algorithm | `Bilinear` | The algorithm to use when resizing the image |

