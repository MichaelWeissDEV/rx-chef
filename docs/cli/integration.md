# Editor integration protocol

`rxchef serve --stdio` is a persistent execution server for Neovim, editor plugins, and local tooling. Descriptor and execution behavior live in `rxchef::integration`, so the same functionality remains directly usable as a Rust library.

## Framing and lifecycle

- stdin/stdout transport with one UTF-8 JSON object per line;
- one compact JSON response for every request containing `id`;
- responses flush immediately;
- requests without `id` are notifications and produce no response;
- the process runs until stdin EOF or `shutdown`;
- no greeting or non-JSON logs are written to stdout;
- protocol version `1`, discoverable with `ping`.

Malformed JSON returns error `-32700` without terminating the process. Both JSON-RPC 2.0 and compact JSONL requests are accepted:

```json
{"jsonrpc":"2.0","id":1,"method":"operations"}
{"id":2,"method":"describe","params":{"operation":"XOR"}}
```

Responses always use JSON-RPC 2.0:

```json
{"jsonrpc":"2.0","id":2,"result":{"name":"XOR","module":"..."}}
```

## Methods

### `ping`

Returns server name, package version, and protocol version.

### `operations`

Returns all complete descriptors with `name`, `module`, `description`, `input_type`, `output_type`, `broken`, and the ordered `args` schema.

### `describe`

```json
{"id":2,"method":"describe","params":{"operation":"from_base64"}}
```

Operation names use the same normalization as the CLI.

### `run`

```json
{"id":3,"method":"run","params":{"operation":"From Base64","input":"SGVsbG8=","args":[]}}
```

Use `input_base64` instead of `input` for exact binary bytes. Providing both is invalid.

### `bake`

```json
{
  "id": 4,
  "method": "bake",
  "params": {
    "input": "Hello",
    "recipe": [
      {"op":"to_upper_case","args":[]},
      {"op":"to_base64","args":[]}
    ]
  }
}
```

`operation` is accepted as an alias for `op`. An empty recipe passes input through unchanged. Step errors identify the one-based step and operation.

### `shutdown`

Returns `{"shutdown":true}`, flushes, and exits normally. Closing stdin is also a clean shutdown.

## Binary-safe result

`run` and `bake` return:

```json
{"output":"display text","output_base64":"exact bytes as Base64","output_len":12}
```

Plugins must decode `output_base64` when exact bytes matter. `output` is UTF-8-lossy and intended for text buffers and previews.

## Error codes

| Code | Meaning |
|---:|---|
| `-32700` | Invalid JSON line |
| `-32600` | Invalid request or JSON-RPC version |
| `-32601` | Unknown method |
| `-32602` | Invalid params or conflicting inputs |
| `-32603` | Internal serialization/registry failure |
| `-32001` | Unknown operation during describe |
| `-32002` | Operation or recipe execution failure |

Protocol errors do not stop the server.

## Neovim process model

Start one process per editor session, keep its pipes open, assign increasing IDs, buffer stdout until newline, and dispatch responses by `id`. Send `shutdown` during teardown and close stdin as fallback. Requests execute sequentially and responses preserve request order.
