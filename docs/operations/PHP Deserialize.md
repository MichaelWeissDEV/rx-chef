# PHP Deserialize

Deserializes PHP serialized data, outputting keyed arrays as JSON.<br><br>This function does not support <code>object</code> tags.<br><br>Example:<br><code>a:2:{s:1:&quot;a&quot;;i:10;i:0;a:1:{s:2:&quot;ab&quot;;b:1;}}</code><br>becomes<br><code>{&quot;a&quot;: 10,0: {&quot;ab&quot;: true}}</code><br><br><u>Output valid JSON:</u> JSON doesn't support integers as keys, whereas PHP serialization does. Enabling this will cast these integers to strings. This will also escape backslashes.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PHP Deserialize"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Output valid JSON | `true` | JSON doesn't support integers as keys, whereas PHP serialization does. Enabling this will cast these integers to strings. This will also escape backslashes. |

