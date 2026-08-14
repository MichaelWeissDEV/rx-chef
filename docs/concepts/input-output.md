# Input and output

rxchef is a byte-oriented Unix filter. Successful payload bytes go to stdout;
diagnostics, counts, and traces go to stderr. Redirecting stdout therefore never
mixes status text into the result.

## Input sources

Commands that execute data accept exactly one explicit source:

- `--input TEXT` supplies the UTF-8 bytes of the command-line value;
- `--input-file PATH` reads the file exactly, including NUL and invalid UTF-8;
- otherwise redirected or piped stdin is read to EOF;
- when stdin is a terminal, no source is considered supplied.

An explicit empty value (`--input ''`), an empty file, or an empty redirected
stdin stream is different from having no source. The execution engine carries
both `bytes` and `supplied` through CLI, library integration, JSONL server, TUI,
and FFI.

Operations declare an `InputRequirement`: `Required` rejects an absent source
but accepts explicit zero bytes; `Optional` accepts either; `Ignored` permits
generators such as UUID creation to run without input. A pipeline output counts
as supplied input for its following step.

## Binary input

Do not put arbitrary binary bytes in shell arguments. Use stdin or
`--input-file`:

```console
rxchef run "From Base64" --input-file encoded.txt --output-file decoded.bin
cat encoded.txt | rxchef run "From Base64" > decoded.bin
```

## Output formats

`run`, `pipe`, `recipe`, and `bake` share `--format` and `--output-file`.

| Format | Contract |
|---|---|
| `raw` | Exact output bytes, unchanged. |
| `text` | UTF-8 text; invalid UTF-8 is an error. |
| `hex` | Lowercase hexadecimal plus a trailing newline. |
| `base64` | Standard padded Base64 plus a trailing newline. |
| `json` | Envelope containing display text and authoritative Base64 bytes. |
| `auto` | Text for valid UTF-8, otherwise a hexadecimal representation. |

Compatibility flags `--hex` and `--json` select the corresponding format and
cannot be combined with a conflicting `--format`.

`--output-file PATH` writes atomically through a sibling temporary file and
renames it only after flush/sync succeeds. It replaces an existing regular file,
fails when the parent is absent or permissions deny the write, and emits no copy
of the payload on stdout.

## Pipes, redirects, and terminals

Raw bytes are the safest automation contract:

```console
base64 < original.bin | rxchef run "From Base64" --format raw > restored.bin
cmp original.bin restored.bin
```

Pipe closure is normal Unix behavior: a broken stdout pipe exits cleanly rather
than printing a panic. stderr remains available for diagnostics. TTY-oriented
`auto` is a presentation convenience; scripts should select `raw`, `text`,
`hex`, `base64`, or `json` explicitly.

For server results, decode `output_base64`; the `output` field is only a lossy
display convenience when `output_is_utf8` is false.

## Byte guarantee

The execution engine, file path, raw stdout, integration API, server Base64
field, and FFI result preserve every byte including `00`, bytes above `7f`, and
invalid UTF-8. Text rendering is the only path that requires valid UTF-8.

## Related pages

- [Data model](data-model.md)
- [Errors and exit codes](errors-and-exit-codes.md)
- [Run](../cli/run.md)
