# Workspace

The workspace layer describes how project state, recipe files, saved pipelines, and user configuration fit together within rxchef.

A workspace is not just the filesystem; it is the conceptual boundary that keeps tool state organized and discoverable for a project or user session.

## Typical contents

A workspace may hold:

- saved pipeline definitions,
- variables,
- configuration metadata,
- recipe files,
- and project-level state used by CLI or TUI flows.

## Why it matters

This abstraction helps users move between one-off operations and reusable project workflows without losing context. It also gives the software a consistent way to persist state outside of the core transformation engine.

## Relationship to the store

The workspace is the user-facing conceptual layer, while the store is the persistence mechanism underneath it. This separation keeps the runtime model clean while still allowing long-lived state.

## Related pages

- [Store](store.md)
- [Project structure](../project/structure.md)
- [Architecture overview](overview.md)
