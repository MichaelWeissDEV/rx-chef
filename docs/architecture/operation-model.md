# Operation model

The operation model defines the common contract each transformation in rxchef follows.

Each operation is expected to expose a consistent set of metadata and behavior:

- a name,
- argument definitions,
- default values,
- a declared input type,
- a declared output type,
- and a transformation function.

## Why a shared model matters

A shared operation contract gives the registry, CLI, and pipeline engine a single place to inspect capabilities. That in turn supports:

- discovery,
- help generation,
- argument validation,
- type coercion,
- and cross-interface execution.

## Typical structure

An operation may define:

- the accepted input shape,
- the operation-specific arguments,
- documentation metadata,
- and the transformation logic itself.

That makes it easy to describe an operation once and use it in many runtime contexts.

## Relation to the registry

The registry collects operations and exposes them by normalized names. The operation model is the schema that allows the registry to sort and display them consistently.

## Related pages

- [Registry](registry.md)
- [Execution engine](execution-engine.md)
- [Architecture overview](overview.md)
