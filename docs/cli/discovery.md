# Operation discovery commands

rxchef exposes two human-oriented and two machine-oriented discovery commands.
All four read the same runtime registry; they differ only in filtering and
representation.

## Command selection

| Command | Intended consumer | Output |
|---|---|---|
| `list [SEARCH]` | Person choosing an operation | Compact names or a JSON name array. |
| `info OPERATION` | Person configuring one operation | Readable metadata/argument schema or JSON. |
| `operations` | UI, editor, generator, inventory | Complete descriptor collection. |
| `operation describe OPERATION` | Machine client configuring one operation | One complete descriptor. |

Machine integrations should use `operations --json` and `operation describe
--json`; their descriptors include facts omitted from the compact list.

## `list`

Syntax:

```text
rxchef list [SEARCH] [--modules] [--json]
```

Examples:

```console
rxchef list
rxchef list base64
rxchef list cipher --modules
rxchef list image --json | jq -r '.[]'
```

`SEARCH` is case-insensitive and matches operation names. Human output contains
one operation per line; `--modules` adds the registry category. JSON output is a
plain array of canonical names and is useful when descriptor details are not
needed.

## `info`

Syntax:

```text
rxchef info OPERATION [--json]
```

Names are normalized, so shell-safe identifiers work:

```console
rxchef info from_base64
rxchef info 'From Base64'
```

Use the schema before constructing a call:

```console
rxchef info 'AES Encrypt' --json \
  | jq '{input_requirement, input_type, output_type, args, availability}'
```

Argument array order is positional order. Each entry identifies its runtime
kind, whether it is required, its default, allowed choices, numeric minimum and
maximum, sensitivity, and description.

## `operations`

Syntax:

```text
rxchef operations [--json] [--search TEXT] [--module MODULE]
                  [--status STATUS] [--all]
```

Examples:

```console
rxchef operations --json
rxchef operations --all --json
rxchef operations --search rsa --json
rxchef operations --module Ciphers --json
rxchef operations --status partial --json
```

By default, entries unavailable in the current feature build are omitted.
`--all` includes them so a UI can present the required feature rather than
making the operation disappear.

The JSON value is an array of descriptors. Important fields are:

```text
name, id, module, description
input_type, output_type, input_requirement
implementation_status, availability
feature_requirements, platform_requirements
side_effects, deterministic, parity
known_limitations, documentation_slug
args[]
```

Filters are applied to descriptors before output. Do not assume the total is a
constant in an external client; count the returned array and react to descriptor
capabilities.

## `operation describe`

Syntax:

```text
rxchef operation describe OPERATION [--json]
```

Example:

```console
rxchef operation describe xor --json | jq '.args[] | {name, kind, default}'
```

Lookup uses the same normalization as `run`. A missing operation is an invalid
command input rather than an empty descriptor.

## Status versus availability

`implementation_status` communicates reviewed implementation maturity.
`availability` communicates whether this compiled binary can execute it.

Examples:

- `partial` + `available`: callable now, conservative maturity classification;
- `partial` + `feature_disabled`: known operation whose optional backend was not
  compiled;
- a future `complete` + unavailable platform/backend: maturity does not make a
  missing dependency executable.

Clients should disable execution using availability and present maturity as
separate information.

## Descriptor-driven UI

A UI can generate a correct form without operation-specific code:

1. call `operation describe`;
2. select controls from `kind`;
3. mark required fields;
4. prefill defaults;
5. use a choice selector when `choices` is non-empty;
6. enforce numeric bounds;
7. hide/redact sensitive values;
8. show feature requirements when unavailable;
9. send ordered arguments to `run` or `bake`.

Refresh descriptors when the binary version changes. The registry is the source
of truth for that binary, while cached schemas may belong to another version.
