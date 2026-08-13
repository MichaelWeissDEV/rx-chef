# Contributing

Thank you for your interest in contributing to rxchef.

The project is maintained by Michael Weiss and is published under the Apache License 2.0.

- [GitHub repository](https://github.com/MichaelWeissDEV/rx-chef)
- [Issue tracker](https://github.com/MichaelWeissDEV/rx-chef/issues)
- [Pull requests](https://github.com/MichaelWeissDEV/rx-chef/pulls)

## Ways to contribute

There are many useful ways to help:

- report bugs,
- improve docs,
- fix operation logic,
- add new operations,
- improve tests,
- refine UX and CLI ergonomics,
- or improve architecture and maintainability.

## Recommended workflow

1. Open or review an issue before starting large changes.
2. Fork the project and create a feature branch.
3. Keep pull requests focused and easy to review.
4. Run the relevant tests before submitting.
5. Include documentation updates when user-facing behavior changes.

## Development setup

```bash
cargo build
cargo test --workspace
```

For documentation work, the MkDocs site can be built locally from the docs folder with the project requirements.

## Code expectations

- prefer clear, idiomatic Rust,
- keep scope small and reviewable,
- add tests when fixing behavior or adding features,
- ensure documentation stays in sync with the code.

## Documentation contributions

This documentation is split into topic-based sections so new content can be added without turning the docs into a monolith. If you are adding a new operation, prefer a dedicated page under the operations section and link it from the relevant category index.

## Licensing

Contributions are expected to align with the Apache 2.0 license used by the project. See [licensing.md](licensing.md) for details.

## Related pages

- [Project overview](overview.md)
- [Project structure](structure.md)
- [Licensing](licensing.md)
