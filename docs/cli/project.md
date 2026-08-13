# Project files

A project combines input, variables, and a pipeline in one JSON/YAML document.

```yaml
name: challenge
data:
  file: samples/payload.txt
variables:
  KEY: hex:2a
pipeline:
  - op: From Base64
    args: []
  - op: XOR
    args: [$KEY, Standard, "false"]
```

Run it with `rxchef project run challenge.yaml --trace`. Relative data paths are
resolved from the project file, not from the caller's current directory. Project
variables participate in normal `$NAME` expansion. The final bytes go to stdout
and trace output goes to stderr.
