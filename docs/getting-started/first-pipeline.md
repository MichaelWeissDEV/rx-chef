# First pipeline

This tutorial shows the shortest path from a raw input to a small composed workflow.

## Goal

We will encode a value, then hash it, and finally inspect the result using a simple pipeline.

## Example

```bash
cargo run -p rxchef_cli -- pipe "to_hex,Space" "sha2,256" --input "Hello"
```

This does the following:

1. converts the text to hexadecimal output,
2. passes that output into the SHA-256 operation,
3. and prints the resulting digest.

## A more explicit example

```bash
echo -n "Hello" | cargo run -p rxchef_cli -- run "To Hex" --input "Hello"
```

Then you can feed that output into another operation or keep the workflow in a saved recipe.

## Why pipelines matter

The pipeline model is one of the most valuable parts of rxchef because it makes multi-step transformations easy to express in one call, while remaining composable with shell tools.

## Next steps

- read the [quickstart](quickstart.md),
- inspect the [CLI overview](../cli/index.md),
- or review the [pipeline concepts guide](../concepts/pipelines.md).
