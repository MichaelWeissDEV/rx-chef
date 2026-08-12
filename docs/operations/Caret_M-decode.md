# Caret/M-decode

Decodes caret or M-encoded strings, i.e. ^M turns into a newline, M-^] turns into 0x9d. Sources such as `cat -v`.

Please be aware that when using `cat -v` ^_ (caret-underscore) will not be encoded, but represents a valid encoding (namely that of 0x1f).

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "Caret/M-decode"`
- Arguments: none

