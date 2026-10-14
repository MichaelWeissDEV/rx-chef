# Multi-input graphs (schema version 1)

The Rust `rxchef::graph` module owns validation and execution. Every data edge
carries exact bytes. Positions are presentation data and never affect scheduling.
A graph can have multiple independent named inputs, fan-out, genuine fan-in,
embedded ordered recipes, and independently named outputs.

Definitions use `.rxgraph.json`. Examples are in `examples/graphs/`:

- `linear`: bytes to Base64.
- `three-inputs`: three binary sources, a shared Base64 round-trip, a concatenation,
  and three outputs (`combined`, `encoded`, `original`).
- `aes-decrypt`: ciphertext, key and IV supplied separately.
- `branch-isolation`: a deliberately invalid AES key fails one branch while the
  original input remains available from another output.

## Ports and arguments

Every producer has a `data` output. Inputs have no inbound ports. Outputs have
one `data` input. Normal operations use `data` plus `arg:<canonical argument ID>`
ports derived from the registry. For example AES Decrypt has `arg:key` and
`arg:iv`. Operation identifiers are canonical registry IDs (`aes_decrypt`).
Literal argument map keys retain exact schema names (`Input`, `Mode`, etc.).
A literal and an edge cannot bind the same argument.

Byte-valued argument ports accept exact bytes, encoded internally using the
shared runtime's explicit typed prefixes. Other argument ports require an
`argument` node declaring the matching registry `ArgKind`. That adapter strictly
interprets UTF-8; schema validation happens before operation dispatch. No lossy
conversion is used for executable data. Text output previews remain display-only.

Combine nodes:

| Kind | Input ports | Result |
|---|---|---|
| `concatenate` | `input:0` through `input:N-1` | Bytes appended in port order |
| `compare` | `left`, `right` | Exactly one byte: `01` equal, `00` unequal |
| `xor` | `left`, `right` | Equal lengths required; mismatches fail |
| `join_text` | Ordered `input:N` | Strict UTF-8 joined using `separator` |
| `select` | Ordered `input:N` | Zero-based `selected`; all ports are dependencies |
| `interleave` | `left`, `right` | Alternate bytes, append remaining bytes |

Recipe nodes call the existing Recipe Engine, preserving Fork/Merge, register,
labels and bounded jumps. Version 1 embeds one ordered recipe level and does
not contain graph references, so recursive graph references cannot be expressed.
`Graph::from_recipe` currently wraps the recipe as one embedded node.
`Graph::to_recipe` accepts only one complete linear chain, including embedded
recipes; it refuses extra inputs, outputs, branches and detached editor nodes.

## Planning and limits

Validation checks the entire definition, including disconnected components.
Cycles, unknown operations, duplicate IDs, incompatible or unknown ports,
multiple inbound connections to a port, invalid arguments and denied host
side effects are rejected. Disconnected editor nodes may lack required
connections but remain idle. Nodes in any output dependency closure must have
all required connections.

Execution computes the requested outputs' dependency closure, schedules using
stable lexical topological order, and executes each node once. Upstream bytes
use immutable `Arc<[u8]>`; operation calls receive owned copies at the existing
registry boundary. Intermediates are released after their last consumer.
Failed dependencies block only their downstream nodes. Successful unrelated
outputs remain complete and independently exportable.

Default limits: 1024 nodes, 4096 edges, depth 256, definition 4 MiB, aggregate
inputs 8 MiB, each node 8 MiB, aggregate outputs 8 MiB, conservative live bytes
32 MiB. The client policy clamps appropriate limits. Output-budget failures
return no payload, never partial bytes. Metadata excludes argument values and
intermediate payloads.

The current scheduler is sequential (bounded to one node at a time). Parallel
branch scheduling remains a release requirement. Local cancellation/deadlines
are checked between nodes and after operation completion. They cannot interrupt
an individual registered operation. Remote graph jobs use one supervised worker
for the whole graph; cancellation/deadlines kill that worker. Hard memory
isolation remains platform dependent as documented in the server guide.

## CLI

```sh
cargo run --locked -p rxchef-cli -- graph validate --graph examples/graphs/three-inputs.rxgraph.json
cargo run --locked -p rxchef-cli -- graph inspect --graph examples/graphs/aes-decrypt.rxgraph.json
cargo run --locked -p rxchef-cli -- graph run \
  --graph examples/graphs/three-inputs.rxgraph.json \
  --input payload=payload.bin --input second=second.bin --input third=third.bin \
  --output combined=combined.bin --output encoded=encoded.bin --output original=original.bin
cargo run --locked -p rxchef-cli -- graph run \
  --graph examples/graphs/three-inputs.rxgraph.json \
  --input payload=payload.bin --select original > original.bin
```

`--json` returns complete per-output results and node metadata. Binary stdout
requires one unambiguous output. Diagnostics use stderr. Any failed requested
output makes the command exit nonzero, even when unrelated outputs succeed.
`--server URL` selects the remote client; `RXCHEF_TOKEN` supplies a credential
without adding it to command-line arguments. `graph list --directory DIR`
lists graph files. Import/export validate definitions and copy to an explicitly
selected `--destination`; they do not yet integrate Store project persistence.

## Server

`POST /api/v1/graphs/validate` and `POST /api/v1/graphs/execute` accept:

```json
{"graph": {}, "inputs_base64": {"payload": "AP/A"}, "outputs": ["original"]}
```

Replace `graph` with a valid definition. `Capabilities.graphs` advertises
support. JSON-RPC methods are `graph.validate` and `graph.execute`. A cancellable
job uses `{"method":"graph_execute","params":{...}}` through the existing jobs
endpoint. Authentication, scope, Host/Origin checks and worker limits apply.
Server requests contain selected bytes, never local file paths.

## Browser and desktop editor

Choose **Graph mode**. Add input/operation/combine/output nodes, edit their names
and parameters in the inspector, and connect handles or use the keyboard-accessible
Source/Target/Port controls. Add up to 64 combine inputs in the inspector. Select
nodes through the canvas or the inspector selector. React Flow provides pan,
zoom, fit-to-view, minimap, selection, movement and keyboard deletion.

**Run graph** uses remote supervised jobs when available. **Cancel graph** calls
the server job API; local desktop calls currently cannot interrupt a running
operation. Output tabs expose bounded HEX/text/Base64 previews and exact binary
downloads. Metadata shows final node status, byte length, elapsed time and error.

Save/load preserves the definition and node positions. It excludes input bytes
and results, so bind inputs again after loading. Browser Save refuses sensitive
literal arguments; supply them through separate named input nodes. Embedded
recipe imports are supported, but advanced recipe/subgraph editing and typed
argument adapter creation still need editor controls. There is no automatic
execution on edits or movement. Persistent projects, live per-node trace payloads,
advanced editing, conversion buttons and TUI graph workflows are not complete.

React Flow was selected using its official MIT license, accessibility and release
documentation: https://reactflow.dev/pro,
https://reactflow.dev/learn/advanced-use/accessibility,
https://github.com/xyflow/xyflow/releases. No externally hosted assets are required.
