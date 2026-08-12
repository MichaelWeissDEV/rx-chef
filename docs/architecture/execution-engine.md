# Execution engine

The execution engine is the runtime layer that turns an operation definition into a concrete transformation over input data.

At a high level it does four things:

1. resolves the operation by name or normalized alias,
2. validates and prepares the supplied arguments,
3. adapts input data to the operation’s expected type,
4. invokes the operation and returns the transformed output.

## Execution flow

A typical execution path looks like this:

- the CLI or library receives a request for an operation,
- the registry resolves the implementation,
- the runtime checks argument counts, defaults, and type constraints,
- the input is normalized into the operation’s declared form,
- the operation runs,
- and the result is returned to the caller.

## Input adaptation

Many operations are designed to work with strings, binary blobs, JSON payloads, or lists. The runtime attempts to coerce inputs into a compatible form without making the operation API awkward to use.

This is especially important in pipelines, where the output of one step may become the input of another. The engine keeps this conversion predictable so that transformations can be chained without manual type handling in every call.

## Error handling

Execution errors should be explicit and actionable. The runtime should preserve enough metadata to explain:

- which operation failed,
- what input type was expected,
- and whether the problem was argument validation, coercion, or execution itself.

This keeps error reporting useful even when a pipeline contains several steps.

## Why this matters

The execution engine is what allows the same operation implementation to be reused in multiple contexts: CLI, library use, pipeline composition, and TUI-driven workflows.

It is the common runtime boundary between the conceptual operation catalog and the practical runtime behavior.

## Related pages

- [Operation model](operation-model.md)
- [Pipeline engine](pipeline-engine.md)
- [Registry](registry.md)
- [Architecture overview](overview.md)
