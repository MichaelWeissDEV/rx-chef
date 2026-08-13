# JSON output

Discovery commands emit ordinary JSON descriptors. Execution commands use a
binary-safe envelope so arbitrary output never becomes invalid UTF-8. The
envelope includes the byte length and either text/JSON-friendly content or a
base64 representation; consumers must follow the encoding field rather than
assuming text.

```console
rxchef operations --json | jq '.[0]'
rxchef run to_base64 --input Hello --json | jq .
rxchef bake --recipe-json '[{"op":"To Hex","args":[]}]' --input Hi --json
```

`scan --json` is NDJSON: each line is a separate finding. `serve --stdio` is
also one JSON request and one JSON response per line. This permits incremental
reading without waiting for EOF. See the [stdio protocol](../cli/integration.md).
