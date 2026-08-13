# Errors and exit codes

`rxchef` keeps successful data on stdout and diagnostics on stderr. This makes
the default output safe to pipe into another process.

| Exit code | Meaning |
|---:|---|
| `0` | The command completed successfully. |
| `2` | `clap` rejected the command line (unknown command/flag, missing value, or conflict). |
| `3` | A command-level input or lookup was invalid. |
| `4` | Operation, recipe, Magic, or server execution failed. |
| `5` | File, stream, project, history, variable, or store I/O failed. |
| `6` | A requested compile-time feature or capability is unavailable. |

Human-readable failures have the form `rxchef: MESSAGE`. With `run --json`,
`pipe --json`, and `bake --json`, successful binary output uses a JSON envelope;
process-level failures still use stderr and a non-zero exit status.

These classes are stable for automation. Detailed messages may gain context and
must not be parsed as a machine protocol; use `serve --stdio` for structured
JSON-RPC errors.

Operation errors are grouped into invalid input, invalid argument, and processing
errors in the Rust API. Callers should display the complete message: it includes
the argument name where one is known.

The stdio server never mixes diagnostics into protocol output. Malformed requests
receive a JSON-RPC error response and the server continues; EOF and the `shutdown`
method end it normally. See [Editor integration](../cli/integration.md).
