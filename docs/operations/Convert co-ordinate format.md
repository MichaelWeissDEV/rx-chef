# Convert co-ordinate format

Converts geographical coordinates between different formats.<br><br>Supported formats:<ul><li>Degrees Minutes Seconds (DMS)</li><li>Degrees Decimal Minutes (DDM)</li><li>Decimal Degrees (DD)</li><li>Geohash</li><li>Military Grid Reference System (MGRS)</li><li>Ordnance Survey National Grid (OSNG)</li><li>Universal Transverse Mercator (UTM)</li></ul><br>The operation can try to detect the input co-ordinate format and delimiter automatically, but this may not always work correctly.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Convert co-ordinate format"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input Format | `Auto` | Format of the input coordinates |
| 2 | Input Delimiter | `Auto` | Delimiter separating the coordinates |
| 3 | Output Format | `Decimal Degrees` | Format to convert to |
| 4 | Output Delimiter | `Space` | Delimiter for the output |
| 5 | Include Compass Directions | `None` | Include N/S/E/W |
| 6 | Precision | `3` | Precision of the result |

