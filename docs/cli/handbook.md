# CLI handbook

This handbook explains how to operate rxchef as a byte-processing tool rather
than merely listing flags. For exact version-specific syntax, pair it with
`rxchef COMMAND --help` and the [complete command reference](reference.md).

## The command model

Every CLI action belongs to one of five groups:

| Goal | Commands |
|---|---|
| Discover capabilities | `list`, `info`, `operations`, `operation describe` |
| Transform data | `run`, `pipe`, `recipe`, `bake` |
| Manage reusable state | `pipeline`, `var`, `history`, `project` |
| Investigate unknown data | `magic`, `scan` |
| Integrate and install | `serve`, `completions`, `manpage` |

The transformation commands share the same operation registry, argument
schemas, execution engine, byte semantics, and structured error model. Choosing
between them is mostly a question of how a recipe is supplied and whether it is
stored.

## Choosing an execution command

| Situation | Best command | Reason |
|---|---|---|
| One known transformation | `run` | Direct operation plus named or positional arguments. |
| Short ad-hoc chain in a shell | `pipe` | Compact left-to-right step syntax. |
| Versioned JSON/YAML workflow | `bake` | Stateless file or inline recipe execution. |
| Stored or project-aware workflow | `recipe` | Resolves files, inline JSON, and saved names. |
| Reusable named workflow | `pipeline run` | Loads the selected Store scope. |
| Editor or long-lived client | `serve --stdio` | Persistent structured protocol without process startup per operation. |

All six paths eventually invoke the same recipe engine. `run` creates a
single-step request; `pipe` parses compact steps; the other commands deserialize
or load a recipe. There is no second implementation with different operation
semantics.

## Discover before executing

The registry is large enough that guessing an operation's spelling or argument
order is unnecessary.

### Human discovery

```console
rxchef list
rxchef list base64
rxchef list hash --modules
rxchef info "AES Encrypt"
```

`list` is deliberately compact. `info` prints the canonical name, category,
input/output types, input requirement, build availability, implementation
status, side effects, and ordered argument schema.

Names are normalized. Case, spaces, dashes, underscores, and CamelCase do not
change lookup identity:

```text
To Base64 = to_base64 = to-base64 = ToBase64
```

If two operations would normalize to the same identity, the registry check
fails during development rather than making runtime selection ambiguous.

### Machine discovery

```console
rxchef operations --all --json > operations.json
rxchef operations --search aes --json
rxchef operations --module Ciphers --json
rxchef operation describe from_base64 --json
```

Use `operations` for integrations because it emits complete descriptors.
Without `--all`, operations unavailable in the current feature build are
omitted; with `--all`, callers can render disabled capabilities and their
requirements. `operation describe` uses the same descriptor representation for
one normalized name.

Do not scrape the human output of `list` or `info`. JSON descriptors are the
stable automation surface.

## Input selection

Execution commands accept one of three byte sources:

1. `--input TEXT` encodes the argument as UTF-8;
2. `--input-file PATH` reads the exact file bytes;
3. otherwise, redirected stdin is read to EOF.

The two explicit flags conflict. If stdin is an interactive terminal and no
flag was supplied, the engine receives “input absent”, not an empty byte slice.
That distinction matters for operations whose schema marks input as required.

```console
# Literal text
rxchef run to_base64 --input 'hello'

# Exact bytes
rxchef run to_base64 --input-file firmware.bin --format raw

# Streaming composition
gzip -c archive.tar | rxchef run to_base64 > archive.tar.gz.b64

# Explicitly supplied empty input
rxchef run sha2 --input ''
```

Shell command substitution cannot preserve NUL bytes. Keep binary data in
pipes or files:

```console
rxchef run from_base64 --input-file payload.b64 --output-file payload.bin
```

## Output selection

The shared `--format` option makes the consumer contract explicit:

| Format | Output |
|---|---|
| `raw` | Exact operation bytes; safest for files and process pipelines. |
| `text` | Valid UTF-8 only; invalid text is an error. |
| `hex` | Lowercase hexadecimal text followed by a newline. |
| `base64` | Standard padded Base64 followed by a newline. |
| `json` | Versioned envelope with display text and authoritative Base64. |
| `auto` | Human-friendly terminal rendering and pipe-safe raw bytes. |

Scripts should choose a format instead of depending on terminal detection.

```console
rxchef run from_base64 --input SGVsbG8= --format text
rxchef run from_base64 --input AP9B --format hex
rxchef run from_base64 --input AP9B --format json | jq -r .output_base64
rxchef run from_base64 --input AP9B --output-file payload.bin
```

`--output-file` uses a sibling temporary file and rename, so consumers do not
observe a half-written result. Successful payload data is never duplicated to
stdout when this option is used.

## Running one operation

The general form is:

```text
rxchef run OPERATION [INPUT OPTION] [--arg NAME=VALUE ...] [POSITIONAL ARG ...]
```

Positional values follow the order printed by `rxchef info`. Named values are
matched case-insensitively and replace the corresponding schema slot:

```console
rxchef run sha2 --input hello --arg Size=256
rxchef run xor --input-file payload.bin \
  --arg Key=hex:deadbeef \
  --arg Scheme=Standard \
  --arg 'Null preserving=false' \
  --output-file decoded.bin
```

Unprefixed arguments are strings. Use an explicit prefix when the schema
expects a different runtime kind:

| Prefix | Parsed value |
|---|---|
| `num:12.5` | Number |
| `bool:true` | Boolean |
| `hex:00ff41` | Bytes decoded from hexadecimal |
| `bytes:00 ff 41` | Bytes decoded from spaced hexadecimal |

Invalid prefixes, choices, bounds, missing required values, and surplus values
fail before operation code is called.

## Building an inline pipeline

`pipe` accepts one shell argument per step:

```console
printf 'hello' | rxchef pipe to_upper_case to_base64
rxchef pipe 'from_hex,Auto' 'xor,hex:2a,Standard,false' \
  --input-file message.hex --output-file decoded.bin
```

Within a step, commas separate the operation name and arguments. Quote or
escape commas that belong to a value:

```console
rxchef pipe 'find_replace,"a,b",Simple string,x' --input 'a,b'
rxchef pipe 'find_replace,a\,b,Simple string,x' --input 'a,b'
```

The shell parses first, then rxchef parses compact-step syntax. Wrapping the
whole step in single quotes avoids most shell expansion surprises.

`--trace` sends step diagnostics to stderr, preserving final stdout:

```console
printf hello | rxchef pipe to_upper_case to_base64 --trace \
  > result.txt 2> trace.txt
```

With JSON output, trace entries are fields in the envelope and remain
binary-safe.

## Recipes and `bake`

A versioned recipe is the durable form of a pipeline:

```yaml
version: 1
name: decode-and-normalize
description: Decode a transport payload and normalize text
steps:
  - op: From Base64
    args: []
  - op: Decode text
    args: [UTF-8]
  - op: To Upper Case
    args: []
```

Run a file without touching the Store:

```console
rxchef bake --recipe recipe.yaml --input-file message.txt --format text
```

Or pass an inline JSON recipe:

```console
rxchef bake \
  --recipe-json '[{"op":"To Upper Case"},{"op":"To Base64"}]' \
  --input hello
```

`recipe FILE_OR_NAME` adds saved-name and project-aware resolution. Use `bake`
for deterministic automation whose complete definition is already supplied;
use `recipe` when Store resolution is intended.

Recipe validation happens before execution. Unknown operations, malformed
control blocks, invalid argument schemas, unsupported versions, duplicate
labels, and impossible jumps fail without running an earlier prefix of the
recipe.

## Flow-control operations

Recipes can describe more than a linear list:

- `Fork` splits input and runs the enclosed block per branch;
- `Merge` closes `Fork` or `Subsection` and combines results;
- `Subsection` transforms regex matches while preserving surrounding bytes;
- `Register` exposes capture values as `$R0`, `$R1`, and later arguments;
- `Label`, `Jump`, and `Conditional Jump` provide bounded control flow.

Blocks can nest. Registers are copied into branches so one branch cannot leak
captures into another. Backward jumps count against the execution step budget,
which prevents a supplied recipe from looping indefinitely.

## Saved pipelines

Saved pipelines are recipes with a name, scope, and management lifecycle:

```console
rxchef project init
rxchef pipeline new transport-decode --description 'Decode captured payloads'
rxchef pipeline add transport-decode from_base64
rxchef pipeline add transport-decode gunzip
rxchef pipeline show transport-decode
rxchef pipeline run transport-decode --input-file capture.txt \
  > payload.bin
```

Editing uses one-based indexes because these are user-facing commands:

```console
rxchef pipeline set transport-decode 1 Alphabet 'A-Za-z0-9+/='
rxchef pipeline remove transport-decode 2
rxchef pipeline rename transport-decode decode-capture
```

Exported JSON/YAML is the sharing boundary:

```console
rxchef pipeline export decode-capture --format yaml --output recipe.yaml
rxchef pipeline import recipe.yaml --name imported-copy --global
```

Project scope wins over global scope when both contain the same name. Select a
scope explicitly for scripts that mutate state.

## Variables and secrets

Variables expand only in operation arguments, never in input bytes or operation
names:

```console
rxchef var set KEY hex:001122 --secret --global
rxchef var set MODE CBC --project
rxchef pipe 'aes_encrypt,$KEY,hex:00000000000000000000000000000000,$MODE' \
  --input-file plaintext.bin
```

Resolution order is invocation override, project, then global:

```console
rxchef pipeline run decrypt --set KEY=hex:aabbccdd --input-file payload.bin
```

`--set` is repeatable, strict about `NAME=VALUE`, and never persists. Secret
metadata controls display redaction; it is not encryption at rest. Use OS file
permissions, protected user profiles, or an external secret manager for the
storage threat model you require.

## History

`--save` on `pipe`, `recipe`, or `pipeline run` stores the recipe steps,
metadata, lengths, and bounded previews:

```console
rxchef pipe to_base64 --input hello --save
rxchef history list
rxchef history show RUN_ID
```

History intentionally does not retain executable input. Replay therefore
requires replacement bytes:

```console
rxchef history run RUN_ID --input replacement
rxchef history run RUN_ID --input-file replacement.bin --trace
```

A preview is display data and is never silently promoted into execution input.
This avoids truncated, lossy, or secret-redacted text changing behavior.

## Projects

`project init` creates `.rxchef/` in the current directory. Commands started in
descendants discover the closest ancestor store, which makes project state
follow a source tree without requiring a global working-directory setting.

`project run FILE` executes a self-contained JSON/YAML challenge definition
with data, variables, and steps. Relative file input resolves against the
project file, not the caller's current directory.

Use `RXCHEF_HOME` to isolate global state in tests or portable installations:

```console
RXCHEF_HOME=/tmp/rxchef-clean rxchef var list
```

## Magic

Magic explores plausible decoding chains and ranks results:

```console
rxchef magic --input U0dWc2JHOD0=
rxchef magic --input-file sample.txt --decode > decoded.bin
rxchef magic --input-file sample.txt --crib 'flag\{[^}]+\}' --json
```

Use the resource options for untrusted or very large data:

```console
rxchef magic --input-file capture.bin \
  --depth 4 \
  --max-candidates 512 \
  --max-candidate-bytes 8388608 \
  --max-total-decoded-bytes 33554432
```

`--intensive` widens the decoder set and increases work. `--decode` emits only
the best candidate and fails when none exists, which is appropriate for shell
automation.

## Scan

Scan tokenizes streams or files, classifies plausible encoded regions, and can
run Magic for each finding:

```console
rxchef scan memory.bin --decode
rxchef scan logs/ --recursive --kind base64,hex --json | jq -c .
tcpdump -r capture.pcap -w - | rxchef scan --entropy 4.5 --max-findings 500
```

JSON mode is NDJSON: each output line is an independent object. This allows
streaming consumers to process findings without holding the complete scan in
memory.

`--max-token-size` bounds retained token bytes and `--max-findings` bounds the
whole traversal. A directory requires `--recursive` before descendants are
read.

## Persistent server

`serve --stdio` is for clients that need repeated discovery and execution:

```console
rxchef serve --stdio --max-request-bytes 1048576
```

stdin and stdout become a protocol channel. Do not print prompts into it. One
UTF-8 JSON object is read per line; one JSON-RPC response is written and flushed
for each request carrying an ID. Notifications produce no response. EOF and
`shutdown` end the process normally.

For exact bytes, clients send `input_base64` and decode `output_base64`. The
display-oriented `output` field is not the binary transport.

## Completion and man pages

Generated shell completion is sourced from the Clap command tree:

```console
rxchef completions bash > ~/.local/share/bash-completion/completions/rxchef
rxchef completions zsh > ~/.zfunc/_rxchef
rxchef completions fish > ~/.config/fish/completions/rxchef.fish
rxchef completions powershell > rxchef.ps1
```

Generate the manual page similarly:

```console
rxchef manpage --output rxchef.1
man ./rxchef.1
```

Because both artifacts are generated from the parser, new flags cannot require
a separately maintained completion schema.

## Failure handling

stdout contains successful data; diagnostics use stderr. Stable exit classes
are:

| Code | Class |
|---:|---|
| 0 | Success |
| 2 | Command-line syntax rejected by Clap |
| 3 | Invalid command input, lookup, or argument |
| 4 | Operation or recipe execution failure |
| 5 | File, Store, project, variable, history, or stream I/O failure |
| 6 | Requested optional capability unavailable in this build |

Do not parse human error text in integrations. Use process status for shell
automation and JSON-RPC error objects for persistent clients.

## Shell-safe operating rules

- Quote operation names containing spaces.
- Put compact pipeline steps in single quotes.
- Use files or OS pipes for binary data.
- Select `--format` explicitly in scripts.
- Redirect traces separately from final stdout.
- Use `--yes` only when deletion is intentionally non-interactive.
- Use explicit `--project` or `--global` on state-changing automation.
- Prefer `operations --json` and server descriptors over scraping prose.
- Bound Magic, Scan, recipe steps, and request size for untrusted input.

## Where to continue

- [Complete command reference](reference.md)
- [Input and output](../concepts/input-output.md)
- [Recipe formats and flow control](recipes.md)
- [Editor integration protocol](integration.md)
- [Storage model](../concepts/storage.md)
- [Platform support](../getting-started/platforms.md)
