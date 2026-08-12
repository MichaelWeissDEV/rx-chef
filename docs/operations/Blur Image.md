# Blur Image

Applies a blur effect to the image.<br><br>Gaussian blur is much slower than fast blur, but produces better results.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Blur Image"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Amount | `5` | The amount of blur to apply. |
| 2 | Type | `Fast` | The type of blur to apply (Fast or Gaussian). |

