# Escape string

Escapes special characters in a string so that they do not cause conflicts. For example, <code>Don't stop me now</code> becomes <code>Don\'t stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\n</code> (Line feed/newline)</li><li><code>\r</code> (Carriage return)</li><li><code>\t</code> (Horizontal tab)</li><li><code>\b</code> (Backspace)</li><li><code>\f</code> (Form feed)</li><li><code>\xnn</code> (Hex, where n is 0-f)</li><li><code>\\</code> (Backslash)</li><li><code>\'</code> (Single quote)</li><li><code>\&quot;</code> (Double quote)</li><li><code>\unnnn</code> (Unicode character)</li><li><code>\u{nnnnnn}</code> (Unicode code point)</li></ul>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Escape string"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Escape level | `Special chars` | The level of escaping to perform |
| 2 | Escape quote | `Single` | Which type of quote to escape |
| 3 | JSON compatible | `false` | Whether to ensure the output is JSON compatible |
| 4 | ES6 compatible | `true` | Whether to use ES6 unicode escape sequences (\\u{...}) |
| 5 | Uppercase hex | `false` | Whether to use uppercase hex digits |

