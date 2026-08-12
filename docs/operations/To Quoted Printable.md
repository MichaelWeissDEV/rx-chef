# To Quoted Printable

Quoted-Printable, or QP encoding, is an encoding using printable ASCII characters (alphanumeric and the equals sign '=') to transmit 8-bit data over a 7-bit data path or, generally, over a medium which is not 8-bit clean. It is defined as a MIME content transfer encoding for use in email.<br><br>QP works by using the equals sign '=' as an escape character. It also limits line length to 76, as some software has limits on line length.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "To Quoted Printable"`
- Arguments: none

