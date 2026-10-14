# Shared engine clients

`rxchef-client` introduces frontend-independent, serializable execution contracts.
It currently covers capabilities, operation discovery/description/search, recipe
validation, single-operation and recipe execution, binary export, and execution
trace artifacts. Magic, Scan, persistence and jobs are not yet client methods.
See the [product roadmap](../development/product-roadmap.md) for the remaining
scope and release blockers.

## Local execution

`LocalEngineClient` calls the existing Core engine exactly once. `EngineClient`
is synchronous and belongs on a UI worker thread. The present TUI still executes
synchronously; making its event loop responsive during long operations remains
part of M7.

```rust
use rxchef_client::{EngineClient, LocalEngineClient, export_bytes};

let client = LocalEngineClient::default();
let response = client.run("To Base64", &[0, 255], vec![])?;
assert_eq!(export_bytes(&response.result)?, b"AP8=");
# Ok::<(), rxchef_client::ClientError>(())
```

The default policy limits input and each output to 8 MiB, and execution to
100,000 actions. It denies operations whose registry metadata declares network,
filesystem or external-process effects. An optional canonical-ID allowlist
further narrows permitted operations, including inside control-flow blocks.
The local TUI explicitly permits existing host effects to preserve local
operation availability. Existing direct Rust and stdio APIs are unchanged.

These are boundary checks, not OS isolation. A decompressor can allocate before
returning output. There is no hard deadline or job cancellation in this client.

## Exact bytes and traces

`ExecuteRequest.input_base64 = None` means absent input; `Some("")` is an
explicit empty input. `ExecutionResult.output_base64` is authoritative; `output`
is only a lossy convenience preview. `export_bytes` decodes Base64 and verifies
the reported byte count.

`execute_observed` is an additive Core API: it lends a completed step's actual
output to a callback without retaining it or re-executing the recipe. Existing
`execute` and metadata-only trace structures retain their previous signatures.
The public `validate_recipe` checks structure and statically known arguments;
register-dependent arguments are validated during execution.

Clients opt into payload capture with `TraceLimits`. Absolute caps are 4,096
entries, 4,096 bytes per entry and 1 MiB total payload. Caller limits can lower
these caps. Each artifact contains exact prefix bytes as Base64, full output
length, recipe index, duration, control-flow flag and a truncation flag. The
response also reports aggregate truncation. Final output is always separate and
complete within the execution policy.

Trace order is action **completion** order. A Fork's children appear before its
combined result. Repeated branches retain the same original recipe index; do
not zip trace entries with recipe steps. History retains the original recipe structure and uses the last captured action
for each step; a missing capture is explicitly labeled. Failed executions currently return a
typed error without a partial trace. Trace payloads may contain keys or personal
data and must be treated as sensitive output.

## HTTP contract foundation

Build with `--features remote` for `RemoteEngineClient`. The standalone
`rxchef-server` provides authenticated REST, JSON-RPC and WebSocket transports
and supervised cancellable jobs. See the [server guide](../server/configuration.md).
Graph execution uses the shared Rust domain through the same worker manager;
`Capabilities.graphs` advertises this additive API.

| Method | Endpoint under `/api/v1/` | Response |
|---|---|---|
| GET | `capabilities` | `Capabilities` |
| GET | `operations` | Operation descriptors |
| POST | `describe` with `{"operation":"To Base64"}` | Operation descriptor |
| POST | `validate` with `ExecuteRequest` | JSON `null` on success |
| POST | `execute` with `ExecuteRequest` | `ExecuteResponse` |

Non-success HTTP responses carry a `ClientError` object with stable `kind`,
human-readable `message`, and optional one-based recipe `step`. Applications
must match `ErrorKind`, never message text. Validate capabilities before using a
new session; an incompatible protocol or byte encoding is rejected.

The adapter requires HTTPS except for literal loopback IP addresses, rejects
embedded URL credentials, paths, queries and fragments, disables proxy discovery
and redirects, uses a bounded response reader (64 MiB), and requires a nonzero
request timeout. Bearer headers are marked sensitive. It does not persist tokens.
A client timeout is a transport timeout only, not a claim that remote work stops.
