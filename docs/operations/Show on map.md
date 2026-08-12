# Show on map

Displays co-ordinates on a slippy map.<br><br>Co-ordinates will be converted to decimal degrees before being shown on the map.<br><br>Supported formats:<ul><li>Degrees Minutes Seconds (DMS)</li><li>Degrees Decimal Minutes (DDM)</li><li>Decimal Degrees (DD)</li><li>Geohash</li><li>Military Grid Reference System (MGRS)</li><li>Ordnance Survey National Grid (OSNG)</li><li>Universal Transverse Mercator (UTM)</li></ul><br>This operation will not work offline.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "Show on map"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Zoom Level | `13` | Zoom level of the map (0-20) |
| 2 | Input Format | `Auto` | Format of the input coordinates |
| 3 | Input Delimiter | `Auto` | Delimiter separating the coordinates |

