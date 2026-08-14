# Architecture overview

rxchef is organized as a layered system that separates reusable transformation
logic from CLI, TUI, protocol, FFI, and storage adapters. This page is the short
map; [System design](system-design.md) follows requests, errors, bytes, control
flow, persistence, and generated metadata through every layer.

## High-level structure

At a high level the project is divided into:

- the core library (`src/`),
- the CLI crate (`crates/cli/`),
- the TUI crate (`crates/tui/`),
- the storage crate (`crates/store/`),
- and the tests and task runner utilities.

## Core responsibilities

### Core library

The main library owns the operation model, pipeline execution, registries, runtime behavior, and shared transformation logic. This is the foundation for all other interfaces.

### CLI

The CLI exposes the operation registry to the terminal. It supports single-operation execution, pipeline processing, variable handling, saved pipeline projects, and recipe-based workflows.

### TUI

The TUI provides an interactive interface for building and running transformation pipelines without needing to memorize command-line syntax.

### Store

The store layer is responsible for persisted state, project metadata, variables, and recipe-related data.

## How the work flows

A typical rxchef workflow looks like this:

1. An operation is looked up in the registry.
2. Arguments are parsed and validated.
3. Input data is transformed by the operation engine.
4. The output is passed to the next pipeline step or returned to the CLI.
5. Optional diagnostic metadata or trace output is emitted for debugging.

## Why this architecture matters

This design keeps the system modular:

- the same operation logic can be reused from code, CLI, and TUI,
- the pipeline engine remains independent from presentation concerns,
- extension points stay clean for new operations and new interfaces.

## Related pages

- [Execution engine](execution-engine.md)
- [Pipeline engine](pipeline-engine.md)
- [Registry](registry.md)
- [Store](store.md)
- [Scan](scan.md)
- [TUI](tui.md)
- [Workspace](workspace.md)
