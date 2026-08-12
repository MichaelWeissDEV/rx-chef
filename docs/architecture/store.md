# Store

The store layer is responsible for persistent state used by the project workflow.

This can include:

- saved pipeline definitions,
- variables,
- project metadata,
- and state needed by the CLI or TUI.

## Why a store exists

Without a persistence layer, user-level workflow state would disappear after each session. The store makes it easier to reuse pipelines, manage variables, and keep project context between runs.

## Responsibilities

- persist user-created pipeline definitions,
- manage configuration or project state,
- allow retrieval and editing of saved work,
- and keep CLI/TUI behavior consistent across sessions.

## Design intent

The store is intentionally separate from the core transformation engine so that data persistence does not leak into the operation execution logic. This keeps the runtime domain model clean and easier to evolve.

## Related pages

- [Workspace](workspace.md)
- [Architecture overview](overview.md)
- [CLI project](../cli/project.md)
