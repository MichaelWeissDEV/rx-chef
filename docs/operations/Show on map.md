# Show on map

Displays comma-separated coordinates on an OpenStreetMap slippy map. Decimal degrees (DD), degrees/decimal minutes (DDM), and degrees/minutes/seconds (DMS) with N/S/E/W suffixes are converted to decimal degrees. Map tiles require network access in the HTML viewer.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Show on map"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Zoom Level | `13` | Zoom level of the map (0-20) |
| 2 | Input Format | `Auto` | Format of the input coordinates |
| 3 | Input Delimiter | `Auto` | Delimiter separating the coordinates |

