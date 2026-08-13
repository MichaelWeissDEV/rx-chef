# Variables

Operation arguments may reference `$NAME` or `${NAME}`. Resolution uses command
overrides first, then project variables, then global variables. Project values
therefore shadow global values with the same name.

```console
rxchef var set KEY hex:001122 --global
rxchef var set IV hex:00000000000000000000000000000000
printf secret | rxchef pipe 'aes_encrypt,$KEY,$IV,CBC' --set KEY=hex:aabbccdd
```

Manage values with `var set`, `var get`, `var list`, and `var unset`. Mutations
use project scope by default; add `--global` for user-wide values. `--set
NAME=VALUE` is ephemeral and never writes to disk.

Typed prefixes are interpreted after expansion: `num:`, `bool:`, `hex:`, and
`bytes:`. Avoid storing secrets in project variables that will be committed;
prefer an ephemeral `--set` value or a protected global store.
