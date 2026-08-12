# TUI architecture

The TUI provides an interactive terminal interface for building and running flows without requiring a deep understanding of shell syntax.

The presentation layer is intentionally separated from the underlying execution engine so that the same operation logic can be reused from both command-line and interactive modes.

## Responsibilities

The TUI typically focuses on:

- browsing available operations,
- composing pipelines visually,
- inspecting intermediate output,
- and editing workflow state more interactively than a command-line interface.

## Relationship to the core model

The TUI does not own operation logic. Instead, it relies on the same registry and runtime primitives used by the CLI and library, which ensures consistent behavior across interfaces.

## Benefits

- easier experimentation for users,
- faster iteration when exploring transformations,
- lower friction for non-shell-native workflows,
- and a consistent user experience across interfaces.

## Related pages

- [Architecture overview](overview.md)
- [Execution engine](execution-engine.md)
- [Workspace](workspace.md)
