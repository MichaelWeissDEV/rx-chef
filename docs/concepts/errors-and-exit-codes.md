# Errors and exit codes

`rxchef` keeps successful data on stdout and diagnostics on stderr. This makes
the default output safe to pipe into another process.

| Exit code | Meaning |
|---:|---|
| `0` | The command completed successfully. |
| `1` | Input, arguments, operation execution, storage, parsing, or I/O failed. |
| `2` | `clap` rejected the command line (unknown command/flag, missing value, or conflict). |

Human-readable failures have the form `rxchef: MESSAGE`. With `run --json`,
`pipe --json`, and `bake --json`, successful binary output uses a JSON envelope;
process-level failures still use stderr and a non-zero exit status.

Operation errors are grouped into invalid input, invalid argument, and processing
errors in the Rust API. Callers should display the complete message: it includes
the argument name where one is known.

The stdio server never mixes diagnostics into protocol output. Malformed requests
receive a JSON-RPC error response and the server continues; EOF and the `shutdown`
method end it normally. See [Editor integration](../cli/integration.md).
