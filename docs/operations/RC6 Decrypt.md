# RC6 Decrypt

RC6 is a symmetric key block cipher derived from RC5. It was designed by Ron Rivest, Matt Robshaw, Ray Sidney, and Yiqun Lisa Yin to meet the requirements of the AES competition, and was one of the five finalists.<br><br>RC6 is parameterised as RC6-w/r/b where w is word size in bits (any multiple of 8 from 8-256), r is the number of rounds (1-255), and b is the key length in bytes. The standard AES submission uses w=32, r=20. Common word sizes: 8, 16, 32 (standard), 64, 128.<br><br><b>IV:</b> The Initialisation Vector should be 4*w/8 bytes (e.g. 16 bytes for w=32). If not entered, it will default to null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, the PKCS#7 padding scheme is used.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "RC6 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Key |
| 2 | IV | `<empty>` | IV |
| 3 | Mode | `CBC` | Mode |
| 4 | Input | `Hex` | Input format |
| 5 | Output | `Raw` | Output format |
| 6 | Padding | `PKCS5` | Padding scheme |
| 7 | Word Size | `32` | Word size in bits (8-256) |
| 8 | Rounds | `20` | Number of rounds (1-255) |

