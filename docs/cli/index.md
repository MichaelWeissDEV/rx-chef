# Command-line interface

The installed executable is `rxchef`. Help is available at every level:

```console
rxchef --help
rxchef bake --help
rxchef operation describe --help
rxchef serve --help
```

## Human and machine discovery

```console
rxchef list hash
rxchef info "From Base64"
rxchef operations --json
rxchef operation describe from_base64 --json
```

Use `list` and `info` interactively. Integrations should use `operations --json` and `operation describe --json`, whose complete descriptors are shared with the library and stdio server.

## Input and output

Execution commands accept literal `--input`, exact bytes from `--input-file`, or stdin when neither option is present:

```console
rxchef run to_base64 --input Hello
rxchef run detect_file_type --input-file sample.bin
printf Hello | rxchef pipe to_upper_case to_base64
```

Final data goes to stdout. Diagnostics and traces go to stderr. Redirected text output is not given an extra newline, so shell pipelines remain byte-clean.

## Recipes

`recipe` integrates with saved recipes and history. `bake` is the stateless machine-facing form:

```console
rxchef bake --recipe recipe.yaml --input Hello
printf Hello | rxchef bake --recipe-json '[{"op":"to_base64"}]'
```

## Persistent editor integration

```console
rxchef serve --stdio
```

The server accepts JSONL/JSON-RPC requests until EOF and is designed to be started once by an editor plugin. See the [protocol specification](integration.md).
