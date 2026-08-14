# System design

rxchef is one transformation system with several adapters. The library owns
operation discovery and execution; the CLI, TUI, stdio server, Store, and C ABI
translate external inputs into the same internal requests. This chapter follows
one request through those layers and explains the boundaries that keep behavior
consistent.

## Design goals

The architecture is optimized for these properties:

- exact byte preservation from input to output;
- one operation registry and one argument schema per operation;
- one recipe engine for linear and control-flow execution;
- deterministic discovery for humans and machine clients;
- explicit side effects, optional features, and availability;
- bounded work for recipes, Magic, Scan, and server input;
- presentation-independent library APIs;
- persistent state isolated behind a separate Store crate;
- panic containment at the C boundary;
- generated reference documentation that cannot silently drift from metadata.

The project deliberately does not make the CLI parser, terminal renderer, or
JSON transport part of an operation implementation. An operation receives typed
input and arguments and returns typed data or a domain error.

## Workspace boundaries

The workspace is divided by responsibility:

| Component | Location | Responsibility |
|---|---|---|
| Core library | `src/` | Operations, metadata, registry, runtime, recipes, Magic, Scan, integration protocol, FFI. |
| CLI | `crates/cli/` | Clap model, files/stdin/stdout, rendering, Store commands, process exit codes. |
| Store | `crates/store/` | Project discovery, recipes, variables, history, atomic JSON persistence. |
| TUI | `crates/tui/` | Interactive pipeline editing and execution through the core engine. |
| Integration tests | `tests/` and crate-local tests | Cross-module behavior and operation vectors. |
| Developer tasks | `crates/xtask/` | Registry generation, audit, docs generation, benchmarks. |
| Fuzz targets | `fuzz/` | Recipe, Magic, and Scan parser/execution stress entry points. |

The dependency direction is intentional:

```text
CLI ───────┬──> Core
           └──> Store

TUI ───────┬──> Core
           └──> Store

Store ─────────> serialization/filesystem primitives

C caller ──────> Core FFI ──> Core execution
JSON client ───> Core integration server ──> Core execution
Rust caller ───> Core public API ──> Core execution
```

The Core does not depend on CLI or TUI. The Store does not execute operations.
This prevents stateful command behavior from leaking into embedders that want a
pure transformation library.

## The operation model

An operation combines executable behavior with a descriptor. The descriptor is
not optional documentation: it drives validation, discovery, frontends, audits,
and generated pages.

Important descriptor fields include:

- canonical name, stable identifier, module, and description;
- declared input and output data types;
- `InputRequirement` (`Required`, `Optional`, or `Ignored`);
- ordered arguments with kind, requirement, default, choices, numeric bounds,
  sensitivity, and description;
- implementation status and compatibility parity;
- build availability and required Cargo features;
- determinism, side effects, and known limitations.

Implementation status and availability answer different questions. A partial
operation may be present in a minimal build. A mature operation may be absent
because its native backend feature is disabled. Callers must inspect both fields
instead of treating “unavailable” as a correctness rating.

## Registry construction

Operation modules are compiled into a deterministic registry. Lookup accepts a
canonical name and normalized spellings, while descriptors retain the canonical
human-facing form.

Registry generation is an explicit developer task:

```console
cargo xtask generate-registry
cargo xtask check-registry
```

The check fails when generated source differs from the registered modules. This
is preferable to a build script that silently rewrites the checkout, because a
normal compile must be read-only with respect to source files.

At runtime, registry lookup follows this sequence:

1. normalize the caller's name;
2. resolve it to one canonical registry entry;
3. return `UnknownOperation` if no entry matches;
4. inspect availability and feature requirements;
5. instantiate the operation only when execution is requested.

Discovery can include unavailable descriptors so UIs can explain how to enable
them without attempting execution.

## Request normalization

Each frontend has its own syntax, but all produce the same conceptual request:

```text
ExecutionRequest
├── input: Vec<u8>
├── input_supplied: bool
├── recipe: validated steps
├── variables: invocation/project/global context
└── options: trace and resource limits
```

`input_supplied` is separate from `input.is_empty()`. This lets a required-input
operation distinguish a user who supplied an empty file from a user who supplied
nothing. The distinction is retained by CLI flags, redirected stdin, server
parameters, Rust integration calls, TUI execution, and null/non-null FFI input
pointers.

Before the first operation runs, the engine validates the recipe structure and
builds label/block information. Per-step argument parsing then uses descriptor
schemas rather than operation-specific CLI code.

## Argument validation

Argument parsing has three stages:

1. map positional and named values into schema slots;
2. parse runtime kinds and apply defaults;
3. enforce requirements, choices, bounds, and surplus-value rules.

The CLI recognizes typed prefixes before runtime validation. The JSON/Rust APIs
can supply typed values directly. After normalization, operations do not need to
guess whether a string was intended as bytes, a boolean, or a number.

Sensitive metadata is carried with the schema. History rendering and related
diagnostics use it to redact values centrally; individual operation modules do
not implement their own secret-name heuristics.

## Execution engine lifecycle

For a normal operation step, the engine performs:

1. resolve the descriptor and availability;
2. enforce the step's input requirement;
3. resolve `$VARIABLE` and register references in arguments;
4. validate and convert arguments;
5. convert current data to the declared input type where compatible;
6. execute the operation;
7. validate/convert the declared output representation;
8. enforce the maximum output size;
9. record trace data when enabled;
10. make the result the next step's supplied input.

Errors retain the one-based recipe step and operation name. A frontend may add
presentation context, but it does not classify errors by matching message text.

The runtime error variants distinguish:

- unknown operation;
- known but unavailable operation;
- invalid argument;
- operation domain failure;
- output validation failure.

The recipe layer wraps these in a structured step error. The CLI maps error
classes to stable process codes, while the server maps them to JSON-RPC domain
errors.

## Data representation

The execution path is byte-oriented. Operations may declare strings, bytes,
numbers, JSON, or other typed representations, but the recipe boundary can carry
exact binary values without passing through lossy UTF-8.

Frontends choose a presentation only at the edge:

- raw stdout and files preserve bytes;
- text output requires valid UTF-8;
- hex and Base64 convert bytes to printable text;
- JSON uses Base64 as the authoritative payload;
- the C ABI returns a pointer plus exact length and capacity.

This separation prevents terminal convenience from changing library or server
results.

## Recipe planning

A recipe version protects the serialization contract. Version 1 is validated
before execution; the loader also imports supported legacy shapes into the
current representation.

Linear recipes need no special planning beyond schema validation. Control-flow
operations add structural rules:

- `Fork` and `Subsection` open blocks;
- `Merge` closes the nearest compatible block;
- nested blocks form isolated execution regions;
- labels map names to validated step indexes;
- jumps cannot cross forbidden structural boundaries;
- duplicate and missing labels are rejected;
- the global step budget bounds backward jumps.

The engine, not each frontend, interprets these operations. Consequently a
recipe behaves the same through `bake`, saved pipelines, projects, TUI, Rust,
and JSON-RPC.

## Fork and subsection state

Forked branches receive independent data and register state. The enclosed block
runs separately for every split segment, and Merge joins results in original
branch order. A failure either aborts the recipe or preserves the original branch
when the block's explicit ignore-errors option is active.

Subsection matches byte/text regions according to its operation contract,
executes the nested block only for matches, and splices transformed regions back
without modifying unmatched bytes. Match-local registers do not leak across
independent regions.

## Resource enforcement

Resource limits are part of execution options rather than CLI-only checks.
Library and server users therefore receive the same protections.

The main boundaries are:

- maximum executed recipe steps;
- maximum final/intermediate output bytes;
- Magic candidate count;
- Magic candidate and cumulative decoded bytes;
- Scan token size and total findings;
- server request-line bytes.

These are deterministic rejection boundaries. They are not estimates based on
available system memory, so clients can choose policies appropriate to their
environment.

## Magic engine

Magic is a bounded search over plausible decode transformations:

1. detectors inspect the current bytes;
2. applicable decoders generate candidates;
3. scoring evaluates printability, structure, known plaintext, and depth;
4. accepted outputs become nodes for the next depth;
5. candidate and byte budgets stop expansion;
6. ranked paths retain the operation chain that produced each result.

Magic calls normal operations through shared runtime resolution. It does not
carry private copies of Base64, hex, or text decoders. Intensive mode broadens
the candidate set and is therefore explicitly opt-in.

## Scan engine

Scan is a streaming-oriented discovery layer:

1. read bounded chunks from stdin or files;
2. identify token boundaries and offsets;
3. classify encoding candidates and entropy;
4. retain at most the configured bytes per token;
5. optionally invoke Magic with a bounded configuration;
6. emit findings incrementally, including path and offset.

NDJSON output mirrors this incremental model: one finding is one line, so an
external tool can process large inputs without waiting for a final array.

## Store architecture

The Store crate persists three kinds of state:

- named recipes/pipelines;
- variables plus descriptions and sensitivity metadata;
- bounded history entries containing recipe steps, lengths, and previews.

Global state uses the platform user configuration directory or `RXCHEF_HOME`.
Project discovery walks ancestors for the nearest `.rxchef` directory. Reads
merge global and project values, with project entries shadowing equal global
names.

Writes use temporary files and rename to avoid exposing partial JSON. On Unix,
secret-bearing files use owner-only permissions. Secret values are still
plaintext at rest; redaction and permissions are the documented protection, not
application-level encryption.

History previews are display-only. Replay requires replacement input because a
bounded or lossy preview cannot safely reconstruct original bytes.

## CLI adapter

The CLI owns concerns that make sense only for a process:

- Clap parsing and help;
- stdin terminal detection;
- file input and atomic output;
- stdout/stderr separation;
- output format selection;
- Store command dispatch;
- broken-pipe handling;
- process exit-code mapping;
- completion and manpage generation.

It does not call operation structs directly for recipe behavior. Even a
single-step `run` is normalized through shared runtime/execution services.

## TUI adapter

The TUI maintains interactive editor state: selected operation, argument forms,
recipe steps, input, output preview, focus, and terminal events. When the user
executes, it constructs the same recipe request as other frontends.

Unicode-safe cursor and truncation logic operates on character boundaries. Store
history uses the same metadata-aware redaction path as the CLI. There is no
headless TUI execution engine competing with Core semantics.

## Stdio integration server

The server is a transport loop around core integration functions:

1. read one bounded UTF-8 JSON line;
2. parse compact JSONL or JSON-RPC 2.0 shape;
3. validate method and parameters;
4. call discovery or execution;
5. serialize one compact JSON-RPC response;
6. flush immediately;
7. continue after domain/protocol errors.

Notifications omit responses. EOF and `shutdown` are clean termination paths.
stdout is reserved for protocol frames; process diagnostics cannot be mixed into
the channel.

The server is sequential by design. Request order is response order, operation
implementations do not need shared mutable session state, and clients can use IDs
without handling out-of-order completion.

## C ABI

The C layer converts raw pointers into validated Rust slices and argument values.
Every exported entry point checks pointer/length combinations before dereference
and contains Rust panics at the ABI boundary.

Ownership rules are explicit:

- callers retain input buffers;
- constructors allocate `RxChefArgValue` objects;
- `rxchef_run` allocates one `RxChefResult`;
- result bytes and error text belong to that result;
- each allocation is released exactly once by its matching `rxchef_free_*`;
- Rust allocation capacity is retained in the result so deallocation uses the
  original layout.

The ABI is experimental, but memory ownership within the current version is a
real contract and is exercised by Rust and C smoke tests.

## Documentation as a derived interface

The 478 operation pages are generated from runtime descriptors plus the explicit
verification manifest. Machine facts are never copied into hand-maintained
sidecars.

The documentation pipeline checks:

- one page per registry entry;
- current argument names, defaults, choices, bounds, and sensitivity;
- input/output types and requirements;
- implementation/availability distinction;
- explicit correctness and performance classifications;
- deterministic generated index and operation matrix.

Narrative chapters such as this one remain hand-written because architecture,
threat models, and workflow decisions cannot be derived from a field list.

## Extension workflow

Adding an operation crosses several intentional gates:

1. implement the operation trait;
2. declare complete metadata and arguments;
3. add positive and error-focused tests;
4. register explicit correctness evidence;
5. add benchmark evidence or a concrete exclusion rationale;
6. regenerate the registry;
7. regenerate operation pages and the matrix;
8. run default and applicable feature tests.

An implementation with missing metadata or a placeholder success path must fail
the audit. A dependency-heavy backend should be optional and remain visible as
feature-disabled in discovery.

## Platform architecture

Portable behavior lives above a small platform edge:

| Portable core | Platform edge |
|---|---|
| Registry and schemas | Configuration directory selection |
| Operations written in Rust | Optional native OCR/disassembly/YARA libraries |
| Recipe serialization | Dynamic library suffix and loader |
| JSON protocol | Shell quoting and path separators |
| Store merge semantics | Unix file permission calls |
| Exit-code classes | Terminal capabilities and event backend |

Linux uses a complete release container. Windows also has a MinGW cross-build
container, while macOS and Windows native behavior use the shared host gate.
Keeping the platform edge narrow makes differences visible instead of scattering
conditional behavior through operations.

## Invariants worth preserving

Future changes should preserve these architectural invariants:

- frontends do not duplicate recipe semantics;
- metadata is explicit rather than inferred from defaults or descriptions;
- binary data is never silently converted through lossy text;
- missing and explicitly empty input stay distinct;
- unavailable features fail explicitly and remain discoverable;
- process text is not treated as a machine error API;
- secret redaction is metadata-driven;
- generated source and docs change only through explicit developer commands;
- release and benchmark commands do not write reports into versioned source;
- external inputs are bounded before unbounded allocation or iteration.

## Related design chapters

- [Execution engine](execution-engine.md)
- [Operation model](operation-model.md)
- [Pipeline engine](pipeline-engine.md)
- [Registry](registry.md)
- [Store](store.md)
- [Magic](magic.md)
- [Scan](scan.md)
- [TUI](tui.md)
- [FFI](ffi.md)
