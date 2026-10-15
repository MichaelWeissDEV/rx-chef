# HTTP, jobs and protocol compatibility

The shared client/server contract version is **2**. The `/api/v1` path and
JSON-RPC **2.0** envelope are separate version namespaces. Discover `version`
and `capabilities` before execution and update clients/servers together.
Generated TypeScript contracts come from Rust; do not hand-edit generated types.

## Start and authenticate

```sh
cargo build --locked --release -p rxchef-server
target/release/rxchef-server
```

The default address is `127.0.0.1:8787`. Enter the generated session token in the
client, or supply it through `RXCHEF_TOKEN`/a token file at startup. A full token
permits execution; a read token permits only version/capabilities/operation
discovery. Host and Origin checks apply independently of token validity.
See [configuration](configuration.md) for deployment and resource flags.

`GET /api/v1/health` is public. Embedded-web GET/HEAD assets are public in the
`web-ui` build, while the API remains authenticated. WebSocket upgrades may
start unauthenticated to support browsers, but must authenticate promptly before
protected calls. Do not put a credential into an exported project or a URL.

## Requests and exact results

Send JSON and a Bearer Authorization header. A recipe execution request for
`POST /api/v1/execute` is:

<!-- rxchef-example: recipe-request -->
```json
{
  "input_base64": "AP+A",
  "recipe": [{"op": "To Base64", "args": []}],
  "variables": {}
}
```

The source bytes are `00 ff 80`. The To Base64 result is the ASCII bytes
`AP+A`; the response's Base64 transport field consequently encodes those ASCII
bytes again. Decode transport Base64 exactly once, then interpret the operation's
result according to the recipe. `input_base64: ""` means explicitly empty bytes;
`null` means no input source. These can have different operation semantics.

Graph requests contain `graph`, `inputs_base64` keyed by declared input name,
and `outputs` as an array of names. An empty output selection requests all
outputs. Every graph edge carries exact bytes; no server-side path loading is
implied. A graph result can contain both successful outputs and branch errors.
HTTP success alone does not imply every requested output succeeded.

## REST and RPC mapping

| REST | JSON-RPC method |
|---|---|
| GET `/api/v1/version` | `version` |
| GET `/api/v1/capabilities` | `capabilities` |
| GET `/api/v1/operations` | `operations` |
| GET `/api/v1/operations/{id}` | `describe` |
| POST `/api/v1/validate` | `validate` |
| POST `/api/v1/execute` or `/api/v1/recipes/execute` | `execute` |
| POST `/api/v1/graphs/validate` | `graph.validate` |
| POST `/api/v1/graphs/execute` | `graph.execute` |
| POST `/api/v1/projects/validate` | `project.validate` |
| POST `/api/v1/magic` | `magic` |
| POST `/api/v1/scan` | `scan` |
| POST `/api/v1/jobs` | `job.submit` |
| GET `/api/v1/jobs/{id}` | `job.get` |
| POST `/api/v1/jobs/{id}/cancel` | `job.cancel` |

HTTP RPC uses `POST /api/v1/rpc` with an envelope such as
`{"jsonrpc":"2.0","id":1,"method":"capabilities","params":{}}`.
The WebSocket endpoint is `/api/v1/ws`; its authenticated RPC surface adds
`job.subscribe`. Search is available through RPC `search`.
The CLI's persistent `rxchef serve --stdio` JSONL integration is a separate
transport with its own [editor protocol](../cli/integration.md); do not assume
an HTTP RPC envelope can be piped unchanged into it.

## Supervised jobs and cancellation

To submit a graph, POST an EngineTask envelope:
`{"method":"graph_execute","params":{...graph request...}}`.
Recipe execution uses `"method":"execute"` with the recipe request as `params`.
These snake-case task tags differ from dotted RPC method names.

The returned snapshot has an ID and status. Poll that ID until a terminal state:

| State | Meaning |
|---|---|
| `queued` | Accepted, waiting for a worker permit |
| `running` | Worker execution active |
| `succeeded` | Complete result available |
| `failed` | Execution failed; inspect the typed error |
| `cancelled` | Cancellation completed |
| `timed_out` | Deadline enforced |

The last four are terminal. `job.cancel` requests termination and may return a
still-running snapshot while the supervisor kills and reaps the worker. Continue
observing. If cancellation transport fails, show that error and keep tracking the
job; do not fabricate a terminal cancellation. Discard results whose client
revision or connection owner has changed.

`progress` may be null. Graph node events contain status, byte sizes and measured
time, not intermediate payloads or invented percentages. Successful job results
use a TaskResult envelope (`kind` plus `value`). Retention is bounded by count,
bytes and TTL; fetching/subscribing also enforces expiry. Fetch needed results
promptly and persist exact output explicitly. A missing expired ID is not a
reason to rerun a possibly side-effecting task automatically.

## Failure handling

Distinguish authentication failure, policy denial, invalid input/recipe/arguments,
unavailable operations, queue admission, resource limits, transport loss and
execution errors. Preserve typed error information and step context. Avoid
logging submitted payloads or credentials when diagnosing invalid parameters.
A lost response does not prove the job was never accepted; reconnect and inspect
known IDs before submitting duplicates.

Worker resource limits constrain CPU, wall time, memory and IPC. They are not a
complete operating-system sandbox. Platform memory enforcement differs; inspect
the [platform limitations](configuration.md#plattformunterschiede-bei-ressourcenlimits)
before relying on a particular boundary.
