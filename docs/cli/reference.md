# Complete CLI reference

The canonical, version-specific syntax is always available from `rxchef COMMAND --help`. This page describes the behavior and purpose of every command and subcommand.

## `operations`

Lists complete operation descriptors through the stable integration API. `--json` emits canonical name, module, description, types, broken flag, and the ordered argument schema for every operation. This is the recommended discovery command for editor plugins; `list --json` remains a compact name array.

## `operation describe OPERATION`

Resolves normalized names and returns one complete descriptor. Use `--json` for machine-readable output.

## `list [SEARCH]`

Lists registered operation names, optionally filtered case-insensitively by `SEARCH`.

- `--modules`: prepend each operation's module/category.
- `--json`: emit a JSON array of names.

The human format writes the match count to stderr.

## `info OPERATION`

Shows canonical name, module, description, declared input/output types, broken status, and ordered argument schema.

- `--json`: emit the metadata as JSON.

## `run OPERATION [ARG...]`

Runs one operation. See [CLI quick start](index.md) for input selection and [Pipelines and parsing](pipelines.md) for argument types.

- `-i, --input TEXT`: literal UTF-8 input.
- `-f, --input-file PATH`: exact file bytes.
- `--arg NAME=VALUE`: repeatable named operation argument.
- `--set KEY=VALUE`: repeatable variable override.
- `--hex`: render output bytes as hex.
- `-j, --json`: emit an output envelope.

Remaining positional values are operation arguments; use `--input` for literal input.

## `pipe STEP...`

Runs compact steps left-to-right. At least one step is required.

- `-i, --input TEXT`, `-f, --input-file PATH`: select input.
- `-t, --trace`: intermediate results (stderr, or JSON fields with `--json`).
- `--hex`: hex output.
- `-j, --json`: machine-readable final output.
- `--save`: add the run to history.
- `--set KEY=VALUE`: repeatable variable override.

See [Pipelines and parsing](pipelines.md) for the complete STEP grammar.

## `recipe RECIPE`

Runs a JSON/YAML path, saved name, or inline JSON array.

- input: `--input`, `--input-file`, or stdin;
- `--trace`, `--hex`, `--save`, and repeatable `--set KEY=VALUE` behave as for `pipe`.

## `bake`

Executes a recipe directly without importing it into persistent storage.

- exactly one of `--recipe PATH` or `--recipe-json JSON` is required;
- recipes may be step arrays or objects containing `steps` (also `pipeline`);
- file recipes support JSON/YAML based on the extension;
- input comes from `--input`, `--input-file`, or stdin;
- `--hex` renders bytes and `--json` emits a binary-safe result envelope.

## `pipeline`

Manages persistent named recipes.

| Subcommand | Behavior |
|---|---|
| `list` | List merged scopes; `--global`/`--project` filter and `--json` structures output. |
| `show NAME` | Print a recipe; `--format json\|yaml`, with `--json` as JSON shorthand. |
| `new NAME` | Create an empty recipe; accepts `--description` and `--global`. |
| `add PIPELINE STEP [ARG...]` | Append a compact step plus optional additional arguments. |
| `remove PIPELINE INDEX` | Remove a one-based step. |
| `set PIPELINE STEP ARG VALUE` | Set by one-based argument index or schema name. |
| `run NAME` | Run with `--input`/`--input-file`/stdin, `--trace`, `--hex`, `--save`, and `--set`. |
| `delete NAME` | Delete in selected scope; prompts unless `--yes`. |
| `export NAME` | Write JSON/YAML to stdout or `--output FILE`. |
| `import FILE` | Import JSON/YAML, optionally `--name NAME` and `--global`. |
| `rename OLD NEW` | Rename in project or `--global` scope. |

Mutating subcommands use project scope by default.

## `var`

| Subcommand | Behavior |
|---|---|
| `set NAME VALUE` | Store a project variable; accepts `--description` and `--global`. |
| `get NAME` | Print the resolved value only. |
| `list` | List merged values; accepts `--global`, `--project`, and `--json`. |
| `unset NAME` | Remove from project or `--global` scope. |

## `history`

| Subcommand | Behavior |
|---|---|
| `list` | Show recent entries; `--limit N` defaults to 20 and `--json` structures output. |
| `show ID` | Show metadata and per-step previews. |
| `run ID` | Replay with preview input or replacement `--input`; accepts `--trace`. |
| `clear` | Delete all history, prompting unless `--yes`. |

## `magic`

Recursively detects and decodes candidate encodings.

- input: `--input`, `--input-file`, or stdin;
- `--depth N`: maximum decoding-chain depth (default 3);
- `--crib REGEX`: favor candidates matching known plaintext;
- `--intensive`: enable more aggressive decoders;
- `--decode`: emit only the best candidate as pipe-friendly bytes;
- `--hex`: hex-render the selected decode;
- `--json`: emit ranked candidates as JSON.

`--decode` returns an error when no candidate exists.

## `scan [PATH...]`

Streams files or stdin and reports encoded/high-entropy tokens.

- `-r, --recursive`: descend into directories;
- `--min-len N`: minimum token length (default 16);
- `-d, --decode`: decode findings with Magic;
- `--depth N`: Magic depth (default 3);
- `--crib REGEX`: only retain matching decoded content and implies decoding;
- `--entropy BITS`: also report tokens at or above the threshold;
- `--kind KIND[,KIND...]`: restrict detector kinds;
- `-j, --json`: emit one JSON object per finding (NDJSON).

Findings go to stdout and the total goes to stderr. Without paths, scan reads stdin. Directories require `--recursive` to include nested directories.

## `project run FILE`

Loads a YAML/JSON project, resolves inline or relative file input, expands project variables, and executes its pipeline.

- `--trace`: write intermediate results to stderr.

## `serve --stdio`

Starts the persistent newline-delimited JSON transport. It writes no greeting or log text to stdout. One request is read per line, responses are flushed immediately, and the process continues until EOF or `shutdown`. See the [protocol specification](integration.md).
