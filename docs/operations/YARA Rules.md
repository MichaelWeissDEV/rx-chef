# YARA Rules

YARA is a tool developed at VirusTotal, primarily aimed at helping malware researchers to identify and classify malware samples. It matches based on rules specified by the user containing textual or binary patterns and a boolean expression. For help on writing rules, see the <a href='https://yara.readthedocs.io/en/latest/writingrules.html'>YARA documentation.</a>

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "YARA Rules"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Rules | `<empty>` | YARA rules to match against the input |
| 2 | Show strings | `false` | Display the strings that matched |
| 3 | Show string lengths | `false` | Display the lengths of the strings that matched |
| 4 | Show metadata | `false` | Display the metadata for each rule |
| 5 | Show counts | `true` | Display the number of times each rule matched |
| 6 | Show rule warnings | `true` | Display any warnings generated during rule compilation |
| 7 | Show console module messages | `true` | Display any messages from the console module |

