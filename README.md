# rxchef

> **Early Alpha** — This project is in a very early stage of development.

If you've ever used [CyberChef](https://github.com/gchq/CyberChef) and wished you could have the same power directly in your terminal, this is a first attempt at making that happen. CyberChef is an incredibly useful tool, and the goal of rxchef is to bring its operations to the command line as a native Rust library — fast, composable, and scriptable.

This is a **work in progress**. Things may break, APIs may change, and not every operation behaves exactly like its JavaScript counterpart yet. Contributions, bug reports, and ideas are **very** welcome — feel free to open issues or pull requests.

## Features

- **478 operations** ported from CyberChef (hashing, encryption, encoding, compression, parsing, and more)
- Reusable Rust library (`rxchef`) for embedding in other projects
- CLI with single operations, inline pipelines, recipe files, variables, and run history
- Interactive TUI for building and running pipelines visually
- FFI interface (C-compatible) for integration with other languages
- Pipeline engine with automatic type coercion between steps
- **Magic** — a recursive detect-and-decode engine that peels back chained
  encodings (e.g. double Base64 → plaintext) and ranks candidates by entropy,
  printability and an optional known-plaintext crib
- **Scan** — a streaming scanner that finds (and optionally decodes) encoded or
  high-entropy strings across large files, whole directories, or piped stdin,
  emitting newline-delimited JSON for `jq`/`grep` pipelines

## Requirements

- Rust 1.75+ (edition 2021)
- C compiler (for native dependencies like `yara-x`, `capstone`)

## Build

```bash
cargo build --release
```

The release binaries are placed in `target/release/`:

| Binary | Description |
|---|---|
| `rxchef` | CLI (command-line interface) |
| `rxchef_tui` | Interactive terminal UI |

## Test

Run the full test suite:

```bash
cargo test --workspace
```

This executes:

- **1178 operation tests** — one test file per operation in `tests/tests/operations/`
- **12 pipeline integration tests** — roundtrips, type coercion, error propagation
- **8 library unit tests** — pipeline engine internals
- **1 doc-test** — pipeline API example

Run tests for a single operation:

```bash
cargo test -p cyberchef-rust-tests --test operations aes_encrypt::
```

Run only pipeline tests:

```bash
cargo test -p cyberchef-rust-tests --test pipeline
```

## CLI Usage

### List operations

```bash
cargo run -p rxchef_cli -- list
```

Search by name:

```bash
cargo run -p rxchef_cli -- list hash
```

### Show operation details

```bash
cargo run -p rxchef_cli -- info "AES Encrypt"
```

### Run a single operation

```bash
cargo run -p rxchef_cli -- run "From Base64" --input "SGVsbG8="
```

Read from a file:

```bash
cargo run -p rxchef_cli -- run "Detect File Type" --input-file sample.bin
```

Pipe through stdin:

```bash
printf 'hello' | cargo run -p rxchef_cli -- run "To Upper Case"
```

Pass operation arguments:

```bash
cargo run -p rxchef_cli -- run "SHA2" --input "hello" "256"
```

Typed argument prefixes:

- `num:12.5` — number
- `bool:true` / `bool:false` — boolean
- `hex:48656c6c6f` — raw bytes from hex

### Run a pipeline

```bash
cargo run -p rxchef_cli -- pipe "to_hex,Space" "sha2,256" --input "Hello"
```

Operation names are normalized — `to_hex`, `ToHex`, and `"To Hex"` all resolve to the same operation.

Use `--trace` to see output after each step:

```bash
cargo run -p rxchef_cli -- pipe "to_upper_case" "to_base64" --input "hello" --trace
```

### Saved pipelines

```bash
cargo run -p rxchef_cli -- pipeline new my-pipe
cargo run -p rxchef_cli -- pipeline add my-pipe "to_hex" "Space"
cargo run -p rxchef_cli -- pipeline add my-pipe "sha2" "256"
cargo run -p rxchef_cli -- pipeline run my-pipe --input "Hello"
cargo run -p rxchef_cli -- pipeline show my-pipe
cargo run -p rxchef_cli -- pipeline export my-pipe --format yaml
```

### Variables

```bash
cargo run -p rxchef_cli -- var set KEY "secret123"
cargo run -p rxchef_cli -- var list
```

Variables are expanded in pipeline arguments via `$KEY` syntax.

### Magic (recursive detect + decode)

`magic` recursively tries the operations that fit the input, decodes, and
recurses — so layered encodings unwrap in one shot. Results are ranked best
first (higher printability / lower entropy / crib match wins).

```bash
# Double Base64 → recovers "Hello" through both layers
rxchef magic --input "U0dWc2JHOD0="

# Print only the winning plaintext (raw, pipe-friendly)
rxchef magic --input "U0dWc2JHOD0=" --decode

# Rank decodes that contain a known string first
rxchef magic --input "…" --crib "flag{"

# Aggressive decoders (ROT13, Base58/85) and deeper chains
rxchef magic --input "…" --intensive --depth 5
```

Because `--decode` writes only the recovered bytes to stdout, `magic` composes
like any other filter:

```bash
echo -n "SGVsbG8gV29ybGQ=" | rxchef magic --decode | rxchef run "To Upper Case"
```

### Scan (find encoded strings in files and streams)

`scan` walks files, directories (`-r`), or stdin, extracts candidate tokens,
classifies them, and (with `--decode`) runs the Magic engine on each. It streams
in chunks — a token that straddles a read boundary is stitched back together —
so it handles GB-scale PCAPs and RAM dumps without loading them into memory.

```bash
# Find and decode encoded strings in a binary
rxchef scan dump.bin --decode

# Recurse a directory and emit NDJSON, one finding per line, for jq
rxchef scan ./logs -r --decode --json | jq 'select(.kinds[]=="From Hex")'

# Stream stdin and flag high-entropy blobs (possible keys / ciphertext)
cat memory.dump | rxchef scan --entropy 4.5

# Only report findings whose decode matches a crib, across many files
rxchef scan ./captures -r --crib "password"

# Restrict to specific encodings
rxchef scan dump.bin --kind base64,hex --min-len 24
```

Data goes to stdout; progress and the finding count go to stderr, so pipelines
stay clean.

**Notes and current limitations:**

- Standard and URL-safe Base64, Base32, Hex, and JWT segments are all detected.
  Run `magic` directly on a whole JWT (`header.payload.signature`) for a single
  structured decode; `scan` catches the `eyJ…` header/payload segments as they
  stream past and decodes each to JSON.
- Memory stays bounded on huge inputs (chunked reads + a per-token size cap),
  but `--decode` runs the full Magic engine on every candidate token, so on
  multi-GB dumps prefer a first pass *without* `--decode` (or narrow with
  `--kind` / `--min-len` / `--entropy`) and decode the interesting hits after.

### Recipe files

Run a JSON or YAML recipe:

```bash
cargo run -p rxchef_cli -- recipe recipe.json --input "Hello"
```

## Library Usage

```rust
use rxchef::operations;

fn main() {
    let op = operations::get_operation("To Upper Case").unwrap();
    let output = op.run(b"hello".to_vec(), &[]).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "HELLO");
}
```

### Pipeline API

```rust
use rxchef::pipeline::Pipeline;
use rxchef::operation::ArgValue;
use rxchef::operations::get_operation;

let result = Pipeline::new()
    .then(get_operation("To Hex").unwrap(), vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)])
    .then(get_operation("From Hex").unwrap(), vec![ArgValue::Str("Auto".into())])
    .run_text("Hello")
    .unwrap();

assert_eq!(result, "Hello");
```



## Contributing

This is an early alpha and there is a lot to do. Contributions of any kind are welcome:

- Bug reports and feature requests via issues
- Fixing or improving existing operations
- Adding missing CyberChef operations
- Improving documentation and examples
- Writing additional tests

If you're unsure where to start, pick any operation from CyberChef that's missing or marked as broken and give it a try.


## Attribution

Ported from [CyberChef](https://github.com/gchq/CyberChef) by GCHQ, originally written in JavaScript.
