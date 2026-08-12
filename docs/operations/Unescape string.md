# Unescape string

Unescapes characters in a string that have been escaped. For example, <code>Don\'t stop me now</code> becomes <code>Don't stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\n</code> (Line feed/newline)</li><li><code>\r</code> (Carriage return)</li><li><code>\t</code> (Horizontal tab)</li><li><code>\b</code> (Backspace)</li><li><code>\f</code> (Form feed)</li><li><code>\nnn</code> (Octal, where n is 0-7)</li><li><code>\xnn</code> (Hex, where n is 0-f)</li><li><code>\\</code> (Backslash)</li><li><code>\'</code> (Single quote)</li><li><code>\&quot;</code> (Double quote)</li><li><code>\unnnn</code> (Unicode character)</li><li><code>\u{nnnnnn}</code> (Unicode code point)</li></ul>

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Unescape string"`
- Arguments: none

