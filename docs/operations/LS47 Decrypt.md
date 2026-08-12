# LS47 Decrypt

This is a slight improvement of the ElsieFour cipher as described by Alan Kaminsky. We use 7x7 characters instead of original (barely fitting) 6x6, to be able to encrypt some structured information. We also describe a simple key-expansion algorithm, because remembering passwords is popular. Similar security considerations as with ElsieFour hold.<br>The LS47 alphabet consists of following characters: <code>_abcdefghijklmnopqrstuvwxyz.0123456789,-+*/:?!'()</code><br>An LS47 key is a permutation of the alphabet that is then represented in a 7x7 grid used for the encryption or decryption.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "LS47 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Password | `<empty>` | The password used to derive the LS47 key |
| 2 | Padding | `10` | The number of padding characters to remove from the start of the output |

