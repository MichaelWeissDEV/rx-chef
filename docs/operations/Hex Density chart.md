# Hex Density chart

Hex density charts are used in a similar way to scatter charts, however rather than rendering tens of thousands of points, it groups the points into a few hundred hexagons to show the distribution.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Hex Density chart"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Record delimiter | `\\n` | Delimiter between records |
| 2 | Field delimiter | `,` | Delimiter between fields |
| 3 | Pack radius | `25` | Radius of the hexagons |
| 4 | Draw radius | `15` | Radius of the hexagons to draw |
| 5 | Use column headers as labels | `true` | Whether to use the first row as headers |
| 6 | X label | `<empty>` | Label for the X axis |
| 7 | Y label | `<empty>` | Label for the Y axis |
| 8 | Draw hexagon edges | `false` | Whether to draw edges around hexagons |
| 9 | Min colour value | `#ffffff` | Colour for low density |
| 10 | Max colour value | `#000000` | Colour for high density |
| 11 | Draw empty hexagons within data boundaries | `false` | Whether to draw empty hexagons |

