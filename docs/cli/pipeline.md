# Saved pipeline command

`rxchef pipeline` creates and edits reusable recipes. Project scope is the
default for changes; `--global` selects user-wide storage.

```console
rxchef pipeline new decode --description 'Decode and inspect'
rxchef pipeline add decode from_base64
rxchef pipeline add decode to_hex None 0
rxchef pipeline set decode 2 Delimiter Colon
rxchef pipeline show decode
printf SGVsbG8= | rxchef pipeline run decode --trace
```

Step and argument indexes are one-based. `set` accepts an argument index or its
schema name. `remove`, `rename`, `delete --yes`, `import`, and `export` complete
the lifecycle. Use `pipeline COMMAND --help` for the version-specific flags and
[the complete reference](reference.md#pipeline) for a compact table.

The Rust `Pipeline` API also validates registered operation availability,
required arguments, and declared argument kinds before it invokes a step. This
makes typed programmatic pipelines follow the same fundamental contracts as
recipes and CLI execution.
