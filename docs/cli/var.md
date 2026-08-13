# Variables

The `var` command manages project or session variables that can be reused in commands and pipeline definitions.

## Example

```bash
printf '%s' "$KEY" | rxchef var set KEY --stdin --secret
rxchef var list
rxchef var list --show-values
```

## Why variables matter

Variables make commands more reusable, easier to read, and easier to parameterize without hardcoding values into scripts.

Values are hidden by default. Secret values require `--show-secrets` to display.
The `secret` marker provides redaction, not encryption; stored values remain
plaintext on disk. Scope defaults to the discovered project, or global when no
project exists, and can be overridden with `--project` or `--global`.

## Related pages

- [CLI overview](index.md)
- [Pipelines](../concepts/pipelines.md)
- [Project](project.md)
