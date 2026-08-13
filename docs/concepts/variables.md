# Variables

Operation arguments may reference `$NAME` or `${NAME}`. Resolution uses command
overrides first, then project variables, then global variables. Project values
therefore shadow global values with the same name.

```console
rxchef var set KEY hex:001122 --global
rxchef project init
rxchef var set IV hex:00000000000000000000000000000000 --project
printf '%s' "$TOKEN" | rxchef var set TOKEN --stdin --secret --global
printf secret | rxchef pipe 'aes_encrypt,$KEY,$IV,CBC' --set KEY=hex:aabbccdd
```

Manage values with `var set`, `var get`, `var list`, and `var unset`. Inside the
nearest ancestor containing `.rxchef`, mutations default to project scope.
Outside a project they default to global scope. `--project` and `--global`
override that choice. `--set NAME=VALUE` is ephemeral and never writes to disk.

Typed prefixes are interpreted after expansion: `num:`, `bool:`, `hex:`, and
`bytes:`. `var list` hides values by default. `--show-values` reveals only
non-secret values; `--show-secrets` is the explicit opt-in for secret values.

Secret values are stored as plaintext JSON—rxchef does not claim to encrypt
them—but the variable file is created with owner-only permissions on Unix.
Avoid committing project secrets; prefer an ephemeral `--set` value or a
protected global store.

Set `RXCHEF_HOME` to replace the normal global configuration directory. This is
useful for portable installations, isolated automation, and tests.
