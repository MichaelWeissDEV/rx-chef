# Substitute

A substitution cipher allowing you to specify bytes to replace with other byte values. This can be used to create Caesar ciphers but is more powerful as any byte value can be substituted, not just letters, and the substitution values need not be in order.<br><br>Enter the bytes you want to replace in the Plaintext field and the bytes to replace them with in the Ciphertext field.<br><br>Non-printable bytes can be specified using string escape notation. For example, a line feed character can be written as either <code>\n</code> or <code>\x0a</code>.<br><br>Byte ranges can be specified using a hyphen. For example, the sequence <code>0123456789</code> can be written as <code>0-9</code>.<br><br>Note that blackslash characters are used to escape special characters, so will need to be escaped themselves if you want to use them on their own (e.g.<code>\\</code>).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Substitute"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Plaintext | `ABCDEFGHIJKLMNOPQRSTUVWXYZ` | The bytes you want to replace |
| 2 | Ciphertext | `XYZABCDEFGHIJKLMNOPQRSTUVW` | The bytes to replace them with |
| 3 | Ignore case | `false` | If true, the case of the input character is preserved. |

