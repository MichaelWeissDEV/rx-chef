# `serve`: persistent stdio integration

`rxchef serve --stdio` turns stdin/stdout into a long-lived JSON protocol channel.
It is intended for editor plugins, language bindings, and local tools that need
many requests without starting one process per operation.

## Start the server

```console
rxchef serve --stdio
rxchef serve --stdio --max-request-bytes 2097152
```

`--stdio` is required so starting `serve` can never accidentally open an
undocumented network listener. The server does not bind TCP, create a socket,
authenticate users, or daemonize itself.

## Process contract

- stdin: one UTF-8 JSON object per line;
- stdout: one compact JSON-RPC response per request with an ID;
- stderr: process diagnostics only;
- notification: a request without an ID, producing no response;
- lifetime: until stdin EOF or the `shutdown` method;
- ordering: sequential request and response order;
- flushing: every response is flushed immediately.

Do not merge stderr into stdout in a client process launcher.

## Minimal session

```console
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"ping"}' \
  '{"jsonrpc":"2.0","id":2,"method":"operations"}' \
  '{"jsonrpc":"2.0","id":3,"method":"shutdown"}' \
  | rxchef serve --stdio
```

The process emits three response lines and exits after the shutdown response.

## Request limits

The default maximum line is 1 MiB. A larger line receives domain error `-32004`,
is discarded without retaining an unbounded buffer, and does not terminate the
session. Choose a value that includes Base64 expansion when transporting binary
input.

## Binary input and output

Send either `input` for UTF-8 text or `input_base64` for exact bytes. Supplying
both is invalid.

```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "run",
  "params": {
    "operation": "From Base64",
    "input": "AP9B",
    "args": []
  }
}
```

Successful execution includes `output_base64`, `output_len`, and
`output_is_utf8`. Always decode Base64 for binary editor buffers.

## Client lifecycle

A robust client should:

1. spawn one process per client session;
2. keep stdin/stdout pipes open;
3. assign unique IDs;
4. buffer stdout until newline;
5. parse one response object;
6. match the ID even though responses are currently ordered;
7. send `shutdown` during normal teardown;
8. close stdin and terminate the child if shutdown cannot complete.

Protocol errors do not kill the server. A client can correct the next request
without respawning.

The full method schemas, result envelopes, and error codes are in the
[editor integration protocol](integration.md).
