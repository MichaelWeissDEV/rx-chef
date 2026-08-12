# Lorenz

The Lorenz SZ40/42 cipher attachment was a WW2 German rotor cipher machine.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Lorenz"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Model | `SZ40` | SZ40, SZ42a, SZ42b |
| 2 | Wheel Pattern | `KH Pattern` | KH, ZMUG, BREAM, etc. |
| 3 | KT-Schalter | `false` | Enable the SZ42a Klartext feedback switch |
| 4 | Mode | `Send` | Send or Receive |
| 5 | Input Type | `Plaintext` | Plaintext or ITA2 |
| 6 | Output Type | `Plaintext` | Plaintext or ITA2 |
| 7 | ITA2 Format | `5/8/9` | 5/8/9 or +/-/. |
| 8 | Psi1 start | `1` | 1-43 |
| 9 | Psi2 start | `1` | 1-47 |
| 10 | Psi3 start | `1` | 1-51 |
| 11 | Psi4 start | `1` | 1-53 |
| 12 | Psi5 start | `1` | 1-59 |
| 13 | Mu37 start | `1` | 1-37 |
| 14 | Mu61 start | `1` | 1-61 |
| 15 | Chi1 start | `1` | 1-41 |
| 16 | Chi2 start | `1` | 1-31 |
| 17 | Chi3 start | `1` | 1-29 |
| 18 | Chi4 start | `1` | 1-26 |
| 19 | Chi5 start | `1` | 1-23 |
| 20 | Psi1 lugs | `.x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x` | 43 long |
| 21 | Psi2 lugs | `.xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x` | 47 long |
| 22 | Psi3 lugs | `.x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x` | 51 long |
| 23 | Psi4 lugs | `.xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.` | 53 long |
| 24 | Psi5 lugs | `xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.` | 59 long |
| 25 | Mu37 lugs | `x.x.x.x.x.x...x.x.x...x.x.x...x.x....` | 37 long |
| 26 | Mu61 lugs | `.xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...` | 61 long |
| 27 | Chi1 lugs | `.x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..` | 41 long |
| 28 | Chi2 lugs | `x..xxx...x.xxxx..xx..x..xx.xx..` | 31 long |
| 29 | Chi3 lugs | `..xx..x.xxx...xx...xx..xx.xx.` | 29 long |
| 30 | Chi4 lugs | `xx..x..xxxx..xx.xxx....x..` | 26 long |
| 31 | Chi5 lugs | `xx..xx....xxxx.x..x.x..` | 23 long |

