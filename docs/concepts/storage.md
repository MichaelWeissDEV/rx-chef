# Storage, scopes, and persistence

rxchef can operate without persistent state. `run`, `pipe`, `bake`, the Rust
Core APIs, FFI, and the stdio server work with explicit requests. The Store is a
separate layer used only when named pipelines, variables, projects, or history
are requested.

## Stored objects

The Store persists:

| Object | Scope | Format | Purpose |
|---|---|---|---|
| Recipes/pipelines | Global or project | One JSON/YAML file per recipe | Named reusable transformations. |
| Variables | Global or project | `vars.json` | Argument values, descriptions, and secret metadata. |
| History | Global | `history.jsonl` | Bounded run metadata and previews. |
| Project marker | Project | `.rxchef/` directory | Defines the ancestor-discovery boundary. |

Input and output payloads are not general Store objects. History records lengths
and previews, not authoritative replay bytes.

## Global directory

The global directory is resolved with the operating system's standard user
configuration location and an `rxchef` child directory. Typical paths are:

| Platform | Typical base |
|---|---|
| Linux | `$XDG_CONFIG_HOME/rxchef` or `~/.config/rxchef` |
| macOS | the user Application Support/configuration directory plus `rxchef` |
| Windows | the roaming user configuration directory plus `rxchef` |

The exact value comes from the Rust `dirs` platform resolver. To make the path
fully deterministic, set `RXCHEF_HOME`:

```console
RXCHEF_HOME=/srv/rxchef-state rxchef var list
RXCHEF_HOME=/tmp/rxchef-test rxchef pipeline list
```

An empty `RXCHEF_HOME` is ignored. Relative override paths are accepted but make
state depend on the process working directory; absolute paths are recommended.

## Project discovery

`rxchef project init` creates `.rxchef/` in the current directory:

```console
cd repository
rxchef project init
```

For later commands, discovery starts at the current directory and walks toward
the filesystem root. The closest ancestor containing a `.rxchef` directory is
the active project.

```text
/work/project/.rxchef/       active Store
/work/project/src/tools/     current directory
```

This design supports commands run from nested directories without storing a
global “current project”. A nested project marker intentionally shadows a
marker higher in the tree.

Outside a discovered project, the default mutation scope is global. Inside a
project, the default is project scope. A command that explicitly requests
project scope outside a project fails with guidance to run `project init`.

## Scope selection

Use explicit scope flags in automation:

```console
rxchef pipeline list --global
rxchef pipeline list --project
rxchef pipeline new decode --global
rxchef var set KEY value --project
```

Reads that do not specify a scope merge both stores. Project values shadow
global values with the same normalized name. This applies to variable resolution
and saved recipe loading.

For recipes, project scope is searched before global scope. Listing retains
scope metadata so callers can explain where each entry came from.

## Directory layout

A representative layout is:

```text
GLOBAL/
├── recipes/
│   ├── decode_transport.json
│   └── normalize.yaml
├── vars.json
└── history.jsonl

PROJECT/.rxchef/
├── recipes/
│   └── challenge.json
└── vars.json
```

History is global-only. Project directories therefore remain focused on
shareable definitions and project-specific arguments rather than accumulating
every local run.

## Recipe files

Saved recipes use version 1 and contain name, description, steps, and optional
tags. New/edited Store recipes are written as pretty JSON. Import accepts JSON,
YAML, and bare arrays of steps; export can produce either JSON or YAML.

Recipe names are converted to safe file names by lowercasing and replacing
characters other than letters, digits, `-`, and `_` with `_`. Lookup first checks
the sanitized file name, then scans recipe content for a case-insensitive display
name match.

The display name remains part of the document. File-name sanitization does not
change operation names or recipe semantics.

## Variables

Variables are stored in a JSON map keyed by uppercase name:

```json
{
  "KEY": {
    "name": "KEY",
    "value": "hex:001122",
    "description": "Transport key",
    "secret": true
  }
}
```

Names are normalized to uppercase for lookup. Values remain strings until they
are expanded into an operation argument; typed prefixes are interpreted after
expansion.

Resolution order is:

1. global variables are loaded;
2. project variables replace equal global names;
3. invocation `--set NAME=VALUE` overrides replace both.

Unknown references remain unchanged rather than expanding to empty text. This
preserves evidence of a missing configuration and lets the receiving schema
reject an invalid value.

## Secret handling

`secret: true` is a display and handling classification:

- normal variable lists do not show values;
- `--show-values` reveals non-secret values only;
- `--show-secrets` is required to print secret values;
- metadata-aware history/trace paths redact sensitive operation arguments;
- Unix variable and history files are created with mode `0600`.

Secret values are plaintext at rest. rxchef does not derive a key, encrypt the
Store, integrate with a keychain, or protect a compromised user account.

For stronger protection:

- supply short-lived values through `--set`;
- pipe a secret into `var set --stdin --secret` instead of exposing it in shell
  history;
- place `RXCHEF_HOME` on protected storage;
- restrict backups and project repository inclusion;
- use an external secret manager to populate the invocation environment.

## Atomic writes

Recipes and variable maps are replaced atomically:

1. create a unique sibling temporary file with `create_new`;
2. write all bytes;
3. flush userspace buffers;
4. synchronize the file;
5. rename the temporary file over the destination;
6. remove the temporary file if an earlier step fails.

Readers therefore see either the previous complete file or the next complete
file, not an intentionally exposed partial JSON document. Atomicity still
depends on ordinary filesystem rename guarantees; do not place the Store on a
filesystem whose rename semantics do not meet your durability requirements.

## History model

Each history line is an independent JSON object containing:

- UUID run ID and timestamp;
- optional saved pipeline name;
- input length and bounded preview;
- step names, redacted arguments, output lengths, previews, durations, errors;
- final output length/preview and success flag.

Text previews replace line breaks for compact display and truncate on Unicode
character boundaries. Binary previews are hexadecimal. Neither is an
authoritative copy of the payload.

History is capped at 10,000 entries. Appending beyond the limit rewrites the file
with the newest 10,000 entries using the atomic replacement path.

Replay always requires replacement input:

```console
rxchef history run RUN_ID --input replacement
rxchef history run RUN_ID --input-file replacement.bin
```

The recorded steps are executable; the recorded preview is not. This prevents a
truncated, lossy, or redacted preview from becoming data without explicit user
choice.

## Concurrency

Store writes are atomic but there is no cross-process transaction or advisory
lock spanning read-modify-write operations. Two processes updating the same
variable map or recipe at the same time may race, with the last complete rename
winning.

For automation that mutates shared state:

- serialize writers externally;
- prefer explicit recipes and `--set` overrides for parallel jobs;
- assign separate `RXCHEF_HOME` directories to isolated workers;
- export and review recipes before replacing shared names.

Read-only execution and the stateless `bake` API do not have this concern.

## Corruption and recovery

Recipe parse failures are returned as Store errors. Invalid variable JSON cannot
be trusted and is treated as an empty variable map by the current loader; restore
or repair `vars.json` rather than continuing with silently missing configuration.
Malformed history lines are skipped during listing so one damaged line does not
hide all later valid records.

A conservative recovery procedure is:

1. stop processes writing the same Store;
2. copy the directory before editing;
3. validate JSON/YAML with an independent parser;
4. restore recipes/variables from version control or backup;
5. run `pipeline list` and `var list` with explicit scope;
6. execute a recipe with known non-secret test input.

## Repository hygiene

`.rxchef/` is ignored by the rxchef repository because local variables and
history-like state do not belong in source control by default. In another
project, choose intentionally:

- share reviewed recipe exports;
- share non-secret project variables only when they are configuration rather
  than credentials;
- never commit global history;
- inspect diffs for secret values and absolute paths;
- prefer a dedicated `recipes/` source directory for artifacts intended to be
  public.

## Stateless alternatives

Use these forms when persistent state is unnecessary:

```console
rxchef run sha2 --input-file artifact.bin
rxchef pipe from_base64 gunzip --input-file payload.txt
rxchef bake --recipe recipe.yaml --input-file data.bin
rxchef serve --stdio
```

Rust callers can use Core without depending on `rxchef_store`. This keeps
embedded applications in control of their own persistence and secret model.

## Related documentation

- [Variables](variables.md)
- [Recipe execution model](recipes.md)
- [CLI handbook](../cli/handbook.md)
- [Project commands](../cli/project.md)
- [History commands](../cli/history.md)
- [Store architecture](../architecture/store.md)
