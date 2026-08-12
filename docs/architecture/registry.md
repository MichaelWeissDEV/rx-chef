# Registry

The registry is the central index of operations available to rxchef.

Whenever a command references an operation by name, the registry resolves the implementation, metadata, and argument schema behind that name. This makes the operation catalog the single source of truth for discovery and execution.

## Role in the system

The registry is the connection between:

- operation definitions in code,
- user-facing command names,
- metadata displayed in help and docs,
- and the runtime execution engine.

By normalizing names and tracking aliases, the registry allows users to call operations in a flexible, forgiving way.

## Key responsibilities

- index operations by canonical and alternate names,
- surface metadata such as argument defaults,
- support help and docs generation,
- resolve operation implementations during runtime,
- and keep CLI and library consumers consistent.

## Related pages

- [Operation model](operation-model.md)
- [Execution engine](execution-engine.md)
- [Architecture overview](overview.md)
