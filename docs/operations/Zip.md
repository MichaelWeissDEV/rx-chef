# Zip

Compresses data using the PKZIP algorithm with the given filename.<br><br>No support for multiple files at this time.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Zip"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Filename | `file.txt` | Name of the file to be zipped |
| 2 | Comment | `<empty>` | Comment to add to the zip file |
| 3 | Password | `<empty>` | Password to protect the zip file (ZipCrypto) |
| 4 | Compression method | `Deflate` | Method to use for compression |
| 5 | Operating system | `Unix` | OS to set in the zip metadata |
| 6 | Compression type | `Dynamic` | Compression level/type (Fixed, Dynamic, None) |

