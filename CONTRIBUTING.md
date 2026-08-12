# Contributing

Thanks for your interest in improving rxchef.

This project is licensed under the Apache License 2.0, and we welcome bug reports, ideas, documentation improvements, and code contributions.

## How to contribute

1. Open an issue before starting significant work for new features or larger refactors.
2. Fork the repository and create a topic branch for your work.
3. Keep changes focused and easy to review.
4. Run the relevant tests before opening a pull request.
5. Submit a pull request with a clear explanation of the change and its motivation.

## Development setup

```bash
cargo build
cargo test --workspace
```

If you work on CLI or TUI behavior, you may also want to run a focused subset of tests for the relevant crate or operation.

## Pull request checklist

- Keep commit history understandable and avoid unrelated changes.
- Include tests for bug fixes or new behavior when practical.
- Update documentation when user-facing behavior changes.
- Ensure formatting and lint expectations are met for Rust code.

## Code style

- Prefer clear, idiomatic Rust.
- Validate behavior with targeted tests before submitting.
- Keep documentation and examples in sync with code changes.

## Reporting bugs

Please include:

- the exact command or reproduction steps,
- the expected behavior,
- the actual behavior,
- relevant logs or output,
- platform and Rust version details if they matter.

## Community

We aim to keep discussion respectful, constructive, and focused on improving the project.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
