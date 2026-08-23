# Quickstart

This tour builds rxchef, discovers operations, transforms text and binary data,
creates a recipe, stores a reusable pipeline, and exercises the machine protocol.
All examples run from the repository root.

## Build the CLI

The minimum build uses only the default feature set:

```console
cargo build --release -p rxchef-cli
```

The executable is:

```text
target/release/rxchef       Linux and macOS
target\release\rxchef.exe   Windows
```

Confirm the parser and version:

```console
target/release/rxchef --version
target/release/rxchef --help
```

For the remaining examples, install it or define a convenient shell variable:

```console
cargo install --path crates/cli
rxchef --version
```

PowerShell users can invoke `.\target\release\rxchef.exe` in place of `rxchef`.

## Find an operation

Start with human-readable discovery:

```console
rxchef list base64
rxchef list hash --modules
rxchef info "From Base64"
```

`info` is important before using an unfamiliar operation. It reports:

- canonical name and category;
- input and output data types;
- whether input is required, optional, or ignored;
- implementation status and availability;
- ordered arguments, defaults, allowed choices, and bounds;
- declared side effects and optional Cargo features.

Operation spelling is normalized, so these resolve identically:

```console
rxchef info 'From Base64'
rxchef info from_base64
rxchef info FromBase64
```

For scripts and plugins, use JSON descriptors instead:

```console
rxchef operations --all --json | jq 'length'
rxchef operation describe from_base64 --json | jq '.args'
```

## Decode text

Supply literal UTF-8 with `--input`:

```console
rxchef run "From Base64" --input "SGVsbG8=" --format text
```

Output:

```text
Hello
```

The same operation can read stdin:

```console
printf '%s' 'SGVsbG8=' | rxchef run from_base64 --format text
```

PowerShell equivalent:

```powershell
'SGVsbG8=' | .\target\release\rxchef.exe run from_base64 --format text
```

Successful payload bytes go to stdout. Diagnostics and traces go to stderr, so
redirecting stdout produces a clean result.

## Preserve binary data

Use files or OS pipes for arbitrary bytes. This roundtrip includes NUL and
non-UTF-8 values:

```console
printf '\000\377A' > original.bin
rxchef run to_base64 --input-file original.bin --format raw > encoded.txt
rxchef run from_base64 --input-file encoded.txt --output-file restored.bin
cmp original.bin restored.bin
```

`--output-file` writes exact bytes atomically and does not duplicate them on
stdout. For automation that transports output through JSON:

```console
rxchef run from_base64 --input AP9B --format json | jq .
```

The JSON `output_base64` field is authoritative. A text/display field may be
lossy when the result is not UTF-8.

## Supply operation arguments

Inspect the schema first:

```console
rxchef info SHA2
```

Use positional arguments in schema order:

```console
rxchef run SHA2 --input hello 256
```

Or name them explicitly:

```console
rxchef run SHA2 --input hello --arg Size=256
```

Named arguments are clearer in long-lived scripts because adding an optional
trailing schema field does not change the meaning of existing names.

Values are strings unless prefixed:

```console
rxchef run XOR --input-file encrypted.bin \
  --arg Key=hex:deadbeef \
  --arg Scheme=Standard \
  --arg 'Null preserving=bool:false' \
  --output-file decrypted.bin
```

Schema validation runs before operation code. Invalid numbers, booleans, hex,
choices, bounds, and missing required arguments return a non-zero status.

## Chain operations

Use multiple processes when ordinary shell composition is clearest:

```console
printf hello \
  | rxchef run to_upper_case \
  | rxchef run to_base64
```

Use one `pipe` process for a compact recipe:

```console
rxchef pipe to_upper_case to_base64 --input hello --format text
```

Steps can carry comma-separated arguments:

```console
rxchef pipe 'to_hex,Space,num:0' 'sha2,256' --input Hello
```

Quote the whole step so the shell passes commas and spaces unchanged. If an
argument itself contains a comma, quote or escape that comma inside the step:

```console
rxchef pipe 'find_replace,"a,b",Simple string,x' --input 'a,b'
```

Trace intermediate results without contaminating final stdout:

```console
rxchef pipe to_upper_case to_base64 --input hello --trace \
  > result.txt 2> trace.txt
```

## Create a recipe file

Save this as `encode.yaml`:

```yaml
version: 1
name: encode
description: Normalize text and encode it for transport
steps:
  - op: To Upper case
    args: []
  - op: To Base64
    args: []
```

Execute it without creating persistent state:

```console
rxchef bake --recipe encode.yaml --input hello --format text
```

Output:

```text
SEVMTE8=
```

Recipes are portable JSON/YAML data. They do not contain shell commands and
therefore avoid platform-specific quoting after parsing.

Inline JSON is useful for short automation:

```console
rxchef bake \
  --recipe-json '[{"op":"To Upper Case"},{"op":"To Base64"}]' \
  --input hello
```

## Store a named pipeline

Initialize project-local state:

```console
rxchef project init
```

Create and edit a saved pipeline:

```console
rxchef pipeline new encode --description 'Project transport encoding'
rxchef pipeline add encode to_upper_case
rxchef pipeline add encode to_base64
rxchef pipeline show encode
rxchef pipeline run encode --input hello
```

Project state lives in the closest ancestor `.rxchef/`. Global state uses the
platform user configuration directory. Project entries shadow global entries of
the same name.

Export the recipe when it should be reviewed or shared:

```console
rxchef pipeline export encode --format yaml --output encode-export.yaml
```

## Add variables

Store reusable argument values rather than copying them into every recipe:

```console
rxchef var set HASH_SIZE 256 --project --description 'Project digest size'
rxchef pipe 'sha2,$HASH_SIZE' --input hello
```

Mark credentials or keys as secret:

```console
printf '%s' 'hex:0011223344556677' \
  | rxchef var set KEY --stdin --secret --project
```

The secret flag redacts normal displays but does not encrypt the JSON file.
Prefer invocation-only overrides for short-lived material:

```console
rxchef pipeline run decrypt \
  --set KEY=hex:aabbccdd \
  --input-file payload.bin > plaintext.bin
```

## Save and inspect history

Add `--save` to a pipeline execution:

```console
rxchef pipe to_upper_case to_base64 --input hello --save
rxchef history list
rxchef history show RUN_ID
```

History stores steps, lengths, durations, and bounded previews. It does not
retain authoritative input. Replay requires replacement bytes:

```console
rxchef history run RUN_ID --input replacement
```

## Investigate unknown input

Magic ranks plausible recursive decodes:

```console
rxchef magic --input U0dWc2JHOD0=
rxchef magic --input U0dWc2JHOD0= --decode
```

Use a crib to favor known plaintext:

```console
rxchef magic --input-file unknown.txt --crib 'flag\{' --json
```

Scan searches files, directory trees, or stdin for candidate tokens:

```console
rxchef scan memory.bin --decode
rxchef scan logs/ --recursive --kind base64,hex --json | jq -c .
```

Both engines expose explicit limits. Lower them when scanning untrusted or very
large data.

## Start the integration server

The server reads one JSON request per line:

```console
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"ping"}' \
  '{"jsonrpc":"2.0","id":2,"method":"run","params":{"operation":"To Base64","input":"hello"}}' \
  '{"jsonrpc":"2.0","id":3,"method":"shutdown"}' \
  | rxchef serve --stdio
```

It remains alive across requests, flushes each response, emits no greeting on
stdout, and exits cleanly on EOF or `shutdown`.

## Generate shell integration

```console
rxchef completions bash > rxchef.bash
rxchef completions zsh > _rxchef
rxchef completions fish > rxchef.fish
rxchef manpage --output rxchef.1
```

Completions and the man page are generated from the same Clap command model as
`--help`.

## Run the project checks

For native development:

```console
./scripts/check-native-platform.sh
```

For the complete platform pipeline:

```console
./scripts/check-platforms.sh
```

The platform guide explains the Linux release container, Windows cross-build
container, and native macOS/Windows stages.

## Next steps

- [CLI handbook](../cli/handbook.md)
- [Complete CLI reference](../cli/reference.md)
- [Recipe execution model](../concepts/recipes.md)
- [Input and output](../concepts/input-output.md)
- [System design](../architecture/system-design.md)
- [Platform support](platforms.md)
- [Rust library](../library.md)
