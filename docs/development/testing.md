# Testing

Run the complete workspace suite:

```console
cargo test --workspace
```

The suite covers all operation modules, typed pipelines, runtime metadata, CLI help, raw stdin/stdout, compact step parsing, direct baking, JSON output, and persistent multi-request JSONL sessions.

Focused checks:

```console
cargo test -p rxchef-cli
cargo test -p rxchef integration::tests --lib
cargo test -p cyberchef-rust-tests --test pipeline
cargo run -p xtask -- docs --check
mkdocs build --strict
```

Whenever an operation changes, regenerate the operation reference using the repository's documentation task and verify that every source operation still has a matching test module.
