# Pipelines

A pipeline is a sequence of operations that run one after another on the same data flow.

## Why pipelines are useful

Pipelines are the natural way to express multi-step transformations in rxchef. They turn a single operation into a reusable workflow and make shell-based automation more readable.

## Example

```bash
cargo run -p rxchef_cli -- pipe "to_hex,Space" "sha2,256" --input "Hello"
```

This composes two steps:

1. convert text to hex,
2. then hash the result with SHA-256.

## Typical pipeline patterns

- decode → inspect → re-encode,
- normalize → filter → transform,
- parse → extract → summarize,
- and scan → decode → confirm.

## Related pages

- [Recipes](recipes.md)
- [Operation arguments](operation-arguments.md)
- [Input and output](input-output.md)
