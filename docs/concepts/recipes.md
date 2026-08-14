# Recipe execution model

A recipe is the portable representation of an rxchef transformation. It is
independent of shell quoting, terminal output, and persistent storage. The same
validated model powers CLI files, inline JSON, saved pipelines, project files,
the TUI, Rust callers, and the stdio server.

## Version 1 document

The preferred YAML shape is:

```yaml
version: 1
name: decode-message
description: Decode and inspect a transported payload
tags: [transport, triage]
steps:
  - op: From Base64
    args: []
  - op: Gunzip
    args: []
  - op: Strings
    args: [All printable chars, "4", Alphanumeric + punctuation]
```

The corresponding JSON is structurally identical:

```json
{
  "version": 1,
  "name": "decode-message",
  "description": "Decode and inspect a transported payload",
  "tags": ["transport", "triage"],
  "steps": [
    {"op": "From Base64", "args": []},
    {"op": "Gunzip", "args": []},
    {
      "op": "Strings",
      "args": ["All printable chars", "4", "Alphanumeric + punctuation"]
    }
  ]
}
```

`version` controls the execution schema, not the package version. Readers can
reject an unsupported future version instead of silently interpreting changed
semantics. Legacy step arrays and supported older object shapes are imported
into version 1 before validation.

## Step structure

Every normal step has:

| Field | Required | Meaning |
|---|:---:|---|
| `op` | yes | Canonical or normalized operation name. |
| `args` | no | Ordered operation arguments; an omitted array is empty. |

`operation` is accepted as an import alias for `op` by the integration model.
Writers should emit `op` so recipes remain concise and canonical.

Operation names are resolved using registry normalization. These all identify
the same entry:

```text
To Base64
to_base64
to-base64
ToBase64
```

Arguments follow the schema order printed by `rxchef info OPERATION`. In CLI
recipe files they are represented as strings and support the same typed prefixes
as direct execution:

```yaml
- op: XOR
  args:
    - hex:deadbeef
    - Standard
    - bool:false
```

Defaults are applied only to omitted trailing/schema slots. An explicit empty
string is a supplied value and may differ from omission.

## Validation before execution

The engine validates structural errors before running the first step. This
prevents a late malformed block from allowing earlier side effects.

Validation covers:

- supported recipe version;
- existence and availability of every operation;
- argument count, requirements, kinds, choices, and bounds;
- block pairing for `Fork`, `Subsection`, and `Merge`;
- label uniqueness and jump targets;
- illegal control-flow crossings;
- global recipe step limits.

Some input-dependent domain checks necessarily occur during execution. For
example, a decoder cannot determine whether bytes are malformed until it sees
them. Those errors still retain the exact step and operation.

## Linear data flow

For a linear recipe, output from step N becomes supplied input to step N+1:

```text
external bytes
    │
    ▼
From Base64
    │ decoded bytes
    ▼
Gunzip
    │ decompressed bytes
    ▼
Strings
    │ text bytes
    ▼
final output
```

The engine carries exact bytes between steps. It consults declared input and
output types for compatible conversions and reports an output-validation error
instead of silently coercing an impossible representation.

An empty recipe is an identity transformation: supplied input is returned
unchanged. It does not manufacture “supplied” input when none existed.

## Input requirements inside a recipe

Each operation declares one of three input requirements:

- `Required`: absent input fails, while explicitly supplied zero bytes are valid;
- `Optional`: both absence and supplied bytes are accepted;
- `Ignored`: the operation derives output from arguments or runtime state.

After a successful step, its result is always supplied input for the next step,
even when the result length is zero. This rule makes generators and filters
compose predictably.

## Variables

Recipe arguments may reference `$NAME` or `${NAME}` when the calling frontend
provides a variable context:

```yaml
version: 1
steps:
  - op: AES Decrypt
    args: [$KEY, $IV, CBC, Raw, Raw, ""]
```

Resolution order is:

1. invocation `--set NAME=VALUE` overrides;
2. project variables;
3. global variables.

Expansion happens before typed-prefix parsing. A variable containing
`hex:001122` therefore becomes bytes when the argument schema expects bytes.
Unknown variables remain literal (`$NAME` or `${NAME}`). The receiving argument
may then reject that literal according to its schema; rxchef never substitutes
an unknown variable with an empty value.

Variables do not expand in operation names, input bytes, file paths, or recipe
metadata. This narrow scope avoids treating arbitrary content as a template.

`recipe`, saved pipelines, and project execution populate the context from
invocation/project/global values. CLI `bake` and the high-level integration
helpers use an empty context; unresolved references remain literal. A Rust host
can provide variables directly with `ExecutionRequest`.

## Registers

The `Register` control operation extracts regex captures from current data and
exposes them as `$R0`, `$R1`, and following indexes to later arguments.

Registers are execution-local:

- they are never written to the Store;
- each fork branch receives an isolated copy;
- each subsection match receives match-local state;
- leaving a block restores the correct parent context.

This makes a recipe deterministic for the same input and variable context.

## Fork and Merge

`Fork` opens a block that runs independently for each split segment. `Merge`
closes the block and joins its results.

```yaml
version: 1
steps:
  - op: Fork
    args: ["\\n", " | ", "false"]
  - op: To Upper case
    args: []
  - op: To Base64
    args: []
  - op: Merge
    args: []
```

Conceptually:

```text
line 1\nline 2
     │ split on newline
     ├── line 1 ──> upper ──> Base64 ──┐
     └── line 2 ──> upper ──> Base64 ──┤
                                       ▼
                                  join with " | "
```

Nested forks are allowed. The validator pairs every Merge with the active block
and rejects missing or surplus Merge steps.

When ignore-errors is enabled for the block, a failing branch keeps its original
segment according to the control operation's contract. Without it, the first
branch failure aborts the complete recipe with branch and step context.

## Subsection and Merge

`Subsection` applies its enclosed block to regex matches and preserves bytes
outside those matches. Merge closes the region in the same structural way as a
Fork block.

This is useful when only embedded values should change:

```text
prefix token=SGVsbG8= suffix
             └───────┘ execute nested decode only here
```

Match offsets are calculated from the current input. Replacements are assembled
without repeatedly slicing already-modified data, so length-changing nested
operations do not corrupt later match positions.

## Labels and jumps

`Label` names a validated location. `Jump` moves execution to a label according
to its configured condition/count, while `Conditional Jump` evaluates its own
condition before changing the program counter.

The planner rejects:

- duplicate labels;
- references to missing labels;
- jumps into or out of incompatible nested blocks;
- malformed label names;
- recipes whose runtime step count exceeds the configured maximum.

Backward jumps are therefore possible but bounded. A recipe cannot create an
unlimited loop merely by jumping to an earlier label.

## Tracing

Tracing records step identity and output preview without changing execution.
CLI human mode writes trace information to stderr; JSON execution returns trace
entries inside the structured envelope.

Trace output is diagnostic:

- it may use previews rather than full data;
- secret arguments are redacted by schema metadata;
- it is not a recipe serialization format;
- it must not be replayed as input.

Use explicit output files or `output_base64` for authoritative bytes.

## Errors

A recipe error retains its location:

```text
step 3 (AES Decrypt): invalid argument 'Key': expected 16, 24, or 32 bytes
```

Error classes distinguish registry lookup, feature availability, argument
validation, operation failure, output validation, control-flow structure, and
resource limits. Frontends render or serialize this structure; they do not infer
classes by searching the message.

No final output is written as success after a failed step. Callers that need
branch-local recovery must choose the explicit ignore-errors block behavior.

## Resource limits

The execution options bound:

- total executed steps, including jumps and branch execution;
- accepted intermediate/final output bytes;
- nested control-flow work;
- frontend-specific request/input limits.

Use lower limits for untrusted recipes. A server can enforce a policy without
changing the recipe file, and a Rust caller can supply limits directly through
`ExecutionOptions`.

## Command selection

The same recipe can enter through several interfaces:

```console
# Stateless file
rxchef bake --recipe decode.yaml --input-file message.txt

# Inline JSON
rxchef bake --recipe-json '[{"op":"From Base64"}]' --input SGVsbG8=

# File, saved name, or inline JSON resolution
rxchef recipe decode.yaml --input-file message.txt

# Imported persistent pipeline
rxchef pipeline import decode.yaml --name decode
rxchef pipeline run decode --input-file message.txt
```

`bake` is the clearest automation interface because it does not consult
persistent recipe storage. `recipe` and `pipeline run` are convenient when
project/global Store resolution is part of the desired behavior.

## Sharing recipes

For reproducible recipes:

- include `version: 1`;
- use canonical operation names in reviewed files;
- commit relative data paths rather than machine-specific absolute paths;
- avoid embedding secrets; reference variables instead;
- document required Cargo features;
- choose arguments explicitly when their defaults are security-sensitive;
- test binary results with hashes or known vectors, not terminal previews;
- keep generated/random operations out of deterministic golden tests unless
  their variability is the subject of the test.

JSON and YAML are equivalent inputs. Exported saved pipelines can be reviewed,
versioned, and imported on Linux, macOS, or Windows without shell-specific
escaping.

## Library execution

Rust callers can use the high-level `rxchef::execute::bake` interface or construct
an `ExecutionRequest` for control over variables, tracing, and limits. The high
level API is appropriate when default safe limits are sufficient; the lower
level API is intended for hosts that own a resource policy.

The stdio server's `bake` method accepts the same step representation and uses
Base64 fields for exact binary input/output. It does not save or import the
recipe into the CLI Store.

## Related documentation

- [CLI recipe formats](../cli/recipes.md)
- [CLI handbook](../cli/handbook.md)
- [Pipelines](pipelines.md)
- [Variables](variables.md)
- [Input and output](input-output.md)
- [Execution engine](../architecture/execution-engine.md)
