# Crop Image

Crops an image to the specified region, or automatically crops edges.<br><br><b><u>Autocrop</u></b><br>Automatically crops same-colour borders from the image.<br><br><u>Autocrop tolerance</u><br>A percentage value for the tolerance of colour difference between pixels.<br><br><u>Only autocrop frames</u><br>Only crop real frames (all sides must have the same border)<br><br><u>Symmetric autocrop</u><br>Force autocrop to be symmetric (top/bottom and left/right are cropped by the same amount)<br><br><u>Autocrop keep border</u><br>The number of pixels of border to leave around the image.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Crop Image"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | X Position | `0` | The x-coordinate of the top-left corner of the crop area |
| 2 | Y Position | `0` | The y-coordinate of the top-left corner of the crop area |
| 3 | Width | `10` | The width of the crop area |
| 4 | Height | `10` | The height of the crop area |
| 5 | Autocrop | `false` | Whether to automatically crop borders |
| 6 | Autocrop tolerance (%) | `2` | The tolerance for color difference when autocropping |
| 7 | Only autocrop frames | `true` | Only crop if all sides have the same border |
| 8 | Symmetric autocrop | `false` | Force autocrop to be symmetric |
| 9 | Autocrop keep border (px) | `0` | The number of pixels of border to leave |

