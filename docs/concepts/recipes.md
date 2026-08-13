# Recipe model

A recipe is an ordered list of operation steps. Each step contains an operation
name and an ordered argument list. Output bytes from one step become the input
bytes of the next step.

```json
{
  "steps": [
    {"op": "From Base64", "args": []},
    {"op": "XOR", "args": ["hex:2a", "Standard", "false"]}
  ]
}
```

Names are normalized (`to_hex`, `to-hex`, `ToHex`, and `To Hex` resolve to the
same operation). Arguments use the schema order shown by `rxchef info OP` and
support the typed prefixes `num:`, `bool:`, `hex:`, and `bytes:`.

Use `recipe` for saved/project workflows, `bake` for stateless files or inline
JSON, and `pipe` for a compact shell expression. See [Recipe formats](../cli/recipes.md).
