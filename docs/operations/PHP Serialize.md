# PHP Serialize

Performs PHP serialization on JSON data.<br><br>This function does not support <code>object</code> tags.<br><br>Since PHP doesn't distinguish dicts and arrays, this operation is not always symmetric to <code>PHP Deserialize</code>.<br><br>Example:<br><code>[5,&quot;abc&quot;,true]</code><br>becomes<br><code>a:3:{i:0;i:5;i:1;s:3:&quot;abc&quot;;i:2;b:1;}<code>

- Input: `JSON`
- Output: `String`
- CLI: `rxchef run "PHP Serialize"`
- Arguments: none

