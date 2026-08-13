# Recipes, saved pipelines, and projects

## Recipe files

`rxchef recipe` accepts a JSON/YAML file, a stored recipe name, or an inline JSON step array.

Full YAML recipe:

```yaml
name: encode-and-hash
description: Encode text, then hash it
steps:
  - op: To Base64
    args: []
  - op: SHA2
    args: ["256"]
tags: [example]
```

Run it with literal input, a file, or stdin:

```console
rxchef recipe recipe.yaml --input Hello
cat input.bin | rxchef recipe recipe.yaml --trace
```

A bare JSON/YAML list of `{op, args}` steps is also accepted. Recipe step arguments use the same typed prefixes and variable expansion as inline pipelines.

## Flow control and registers

All recipe frontends use the same flow-aware library engine: `bake`, `pipe`,
saved recipes, saved pipelines, projects, and the `bake` JSONL method. The
following control operations therefore compose with every normal operation:

- `Fork` splits binary input on its delimiter, executes steps independently
  until the matching `Merge`, and joins branch results with the merge delimiter.
- `Subsection` applies the enclosed steps only to regex matches and preserves
  all surrounding bytes.
- `Register` stores regex capture groups as `$R0`, `$R1`, ... for later step
  arguments. Fork and subsection branches receive isolated register copies.
- `Label`, `Jump`, and `Conditional Jump` support forward and bounded backward
  control flow. Missing/duplicate labels and branch-crossing jumps are errors.

Control blocks may be nested. Every `Fork` and `Subsection` must have a matching
`Merge`. `Ignore errors` preserves the original branch or match when an enclosed
operation fails. The engine also enforces a global execution limit as protection
against accidental infinite loops in externally supplied recipes.

```yaml
- op: Fork
  args: ["\\n", " | ", "false"]
- op: To Upper case
- op: To Base64
- op: Merge
```

`rxchef run Fork` can only pass its input through because a single-operation
call has no following block; use any recipe frontend for flow control.

## Saved pipelines

Project-scope data is stored under `./.rxchef/`; global data is stored in the platform configuration directory. Project entries override same-named global entries when loading.

```console
rxchef pipeline new demo --description 'Example pipeline'
rxchef pipeline add demo to_upper_case
rxchef pipeline add demo to_base64
rxchef pipeline show demo
rxchef pipeline run demo --input Hello --trace
```

Use `--global` on mutating commands to select global scope. `pipeline list --global` and `pipeline list --project` filter the listing.

Pipelines can be edited and moved between installations:

```console
rxchef pipeline set demo 2 Alphabet 'A-Za-z0-9+/='
rxchef pipeline remove demo 1
rxchef pipeline rename demo renamed
rxchef pipeline export renamed --format yaml --output renamed.yaml
rxchef pipeline import renamed.yaml --name imported
rxchef pipeline delete imported --yes
```

Step and argument indexes are one-based. `pipeline set` accepts either an argument index or the schema name shown by `rxchef info OP`.

## History

Add `--save` to `pipe`, `recipe`, or `pipeline run` to record a run:

```console
rxchef pipe to_base64 --input Hello --save
rxchef history list
rxchef history show RUN_ID
rxchef history run RUN_ID --input Replacement
rxchef history clear --yes
```

History stores previews and lengths, not necessarily the complete original input. Replaying without `--input` can therefore differ for a large original input; the CLI warns when this applies.

## Project files

A project combines input data, variables, and a pipeline:

```yaml
name: challenge
data:
  inline: SGVsbG8=
variables:
  FORMAT: Auto
pipeline:
  - op: From Base64
    args: ["$FORMAT"]
  - op: To Upper Case
    args: []
```

```console
rxchef project run challenge.yaml --trace
```

File-backed project input is resolved relative to the project file.
