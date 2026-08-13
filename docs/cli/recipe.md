# Running recipes

`rxchef recipe RECIPE` accepts a JSON/YAML path, a saved pipeline name, or an
inline JSON step array. Input comes from exactly one of `--input`,
`--input-file`, or stdin.

```console
printf Hello | rxchef recipe encode.yaml
rxchef recipe '[{"op":"To Base64","args":[]}]' --input Hello
rxchef recipe saved-name --input-file payload.bin --trace --save
```

Use `--hex` when terminal output must be printable, `--trace` for intermediate
results on stderr, `--set NAME=VALUE` for ephemeral variables, and `--save` to
record the run. For stateless integrations prefer `bake --recipe FILE` or
`bake --recipe-json JSON`. See [Recipe formats](recipes.md).
