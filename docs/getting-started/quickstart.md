# Quickstart

This quickstart gives you the shortest path from a fresh clone to a working rxchef workflow.

## 1. Build the project

```bash
cargo build --release
```

## 2. List available operations

```bash
cargo run -p rxchef_cli -- list
```

Or search for a specific category:

```bash
cargo run -p rxchef_cli -- list hash
```

## 3. Run a single operation

```bash
cargo run -p rxchef_cli -- run "From Base64" --input "SGVsbG8="
```

## 4. Pipe data through a recipe

```bash
printf 'hello' | cargo run -p rxchef_cli -- run "To Upper Case"
```

## 5. Use a pipeline

```bash
cargo run -p rxchef_cli -- pipe "to_hex,Space" "sha2,256" --input "Hello"
```

This is especially useful for chained transformations where output from one step becomes the input for the next.

## 6. Explore the operation catalog

The full operation index is available here:

- [Operations overview](../operations/index.md)
- [Operations reference](../reference/operations.md)

## 7. Read or write recipe files

rxchef supports YAML and JSON recipe definitions, which are useful for repeatable workflows and automation.

## Next steps

- Read the [architecture overview](../architecture/overview.md)
- Learn about [concepts and pipelines](../concepts/pipelines.md)
- See the [development guide](../development/building.md)
