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

## Execution and trace changes

Execution now goes through `LocalEngineClient`. Press `r` to run after editing;
loading, editing or restoring a recipe does not execute it implicitly. The trace
captures real intermediate bytes in one execution. Entries are labeled by their
original recipe index, so repeated Fork steps are not mislabeled as subsequent
recipe operations. Bounded prefixes are marked as truncated, and final output
is a separate view. Binary preview formatting inspects at most 256 bytes.

The event loop still blocks during execution. Async execution, remote sessions,
module separation and broader interaction tests remain on the product roadmap.
