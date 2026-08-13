# FFI

!!! warning "Experimental ABI"

    The C ABI is not part of the stable 1.0 compatibility promise yet. Use the
    Rust API or stdio protocol for stable integration. The checked-in
    `include/rxchef.h` header documents the current experimental surface.

The project includes a foreign-function interface layer so that runtime capabilities can be reused outside the Rust codebase.

This is useful for embedding rxchef in other languages or systems where direct Rust integration is not ideal. The design keeps the FFI surface small and explicit so it remains easier to maintain than exposing the full internal object model.

## Goals

The FFI layer focuses on:

- C-compatible function boundaries,
- straightforward parameter marshaling,
- predictable ownership rules,
- and a limited surface area that is easier to support over time.

## Typical use cases

- embedding rxchef in a native application,
- exposing transformations to other runtimes,
- integrating with tooling that expects a stable C ABI,
- or providing a lower-level interface for automation.

## Design principles

- keep the public ABI minimal,
- avoid exposing internal Rust-only types directly,
- centralize conversion boundaries,
- and keep runtime errors readable and recoverable.

## Relationship to the library

The core library remains the canonical implementation. The FFI layer acts as an adapter: it accepts simpler inputs, passes them into the library layer, and converts the result back into a form that external callers can consume safely.

This separation prevents the internal engine from being tightly coupled to a language boundary.

Returned strings, arguments, and results must be passed exactly once to their
matching `rxchef_free_*` function. Double-free and freeing foreign pointers are
caller contract violations. A non-zero length always requires a non-null input
pointer. Operation panics are caught at the execution boundary and returned as
errors; invalid foreign pointers cannot be made safe by Rust and remain outside
the API contract.

## Related pages

- [Architecture overview](overview.md)
- [Execution engine](execution-engine.md)
- [Workspace](workspace.md)
