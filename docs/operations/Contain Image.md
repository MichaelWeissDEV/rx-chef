# Contain Image

Scales an image to the specified width and height, maintaining the aspect ratio. The image may be letterboxed.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Contain Image"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Width | `100` | The width of the contained image |
| 2 | Height | `100` | The height of the contained image |
| 3 | Horizontal align | `Center` | The horizontal alignment of the image within the container |
| 4 | Vertical align | `Middle` | The vertical alignment of the image within the container |
| 5 | Resizing algorithm | `Bilinear` | The algorithm to use when resizing the image |
| 6 | Opaque background | `true` | Whether to use an opaque black background instead of transparency |

