# Parse SSH Host Key

Parses a SSH host key and extracts fields from it.<br>The key type can be:<ul><li>ssh-rsa</li><li>ssh-dss</li><li>ecdsa-sha2</li><li>ssh-ed25519</li></ul>The key format can be either Hex or Base64.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse SSH Host Key"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input Format | `Auto` | The format of the input key |

