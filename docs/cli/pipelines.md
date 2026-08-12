# Pipelines and parsing

## Shell pipelines

Every `run` and `pipe` invocation can read stdin and emit pipe-clean stdout:

```console
cat payload.txt \
  | rxchef run from_base64 \
  | rxchef run gunzip \
  | rxchef run strings \
  | grep token
```

Each process receives and emits raw bytes. Do not use command substitution for binary data because shells cannot preserve NUL bytes; use an OS pipe or files.

## Inline operation pipelines

`rxchef pipe STEP...` runs any number of registered operations from left to right. The previous output becomes the next input:

```console
printf 'Hello' | rxchef pipe to_upper_case to_base64 from_base64
```

Each compact step has this grammar:

```text
STEP := OPERATION ("," ARGUMENT)*
```

Examples:

```console
rxchef pipe 'to_hex,Space,num:0' 'sha2,256' --input Hello
rxchef pipe 'find_replace,"a,b",Simple string,x' --input 'a,b'
rxchef pipe 'find_replace,a\,b,Simple string,x' --input 'a,b'
```

Single or double quotes inside the STEP group a comma-containing argument. A backslash escapes a comma, the active quote, or another backslash. Backslashes before other characters remain literal, so regex values such as `\d+` survive parsing. An unclosed quote is an error. Shell quoting happens first; enclosing the entire STEP in single quotes is usually the least surprising form.

Empty positional arguments are significant and preserve schema positions:

```console
rxchef pipe 'some_operation,,third-value' --input data
```

## Argument types

Operation arguments are strings by default. Prefix a value when the operation expects another runtime type:

| Prefix | Runtime value | Example |
|---|---|---|
| none | string | `CBC` |
| `num:` | floating-point number | `num:12.5` |
| `bool:` | boolean | `bool:true` |
| `hex:` | decoded bytes | `hex:48656c6c6f` |
| `bytes:` | decoded bytes | `bytes:48 65 6c 6c 6f` |

Hex prefixes accept whitespace and an optional `0x`. Invalid numbers, booleans, or hex fail before the operation runs.

For a single operation, named arguments avoid positional ambiguity:

```console
rxchef run sha2 --input hello --arg Size=256
```

`--arg` is repeatable and matched case-insensitively against `rxchef info OP`. Positional and named arguments can be mixed; named values replace the corresponding schema position.

## Type flow

The Rust `Pipeline` API uses each operation's declared input and output type and performs compatible conversions at step boundaries. The CLI executes the same operations as byte filters, preserving arbitrary bytes between steps. A failure reports the one-based CLI step number and operation name, then exits non-zero without writing a final result.

## Tracing and JSON

`--trace` writes every intermediate result to stderr. The final result remains on stdout:

```console
printf hello | rxchef pipe to_upper_case to_base64 --trace >result.txt 2>trace.txt
```

With `--json --trace`, the trace is included in the JSON envelope instead of producing human trace text. Binary-safe final output is always available as Base64.

## Variables

Saved variables use `$NAME` in operation arguments:

```console
rxchef var set KEY secret123
rxchef pipe 'hmac,SHA256,$KEY' --input message
rxchef pipe 'hmac,SHA256,$KEY' --set KEY=temporary --input message
```

Names are normalized to uppercase. Repeated `--set KEY=value` overrides stored values for that invocation, and values may contain additional `=` characters.
