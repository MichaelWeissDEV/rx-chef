# Storage scopes

rxchef has two persistent scopes. Project state lives in `.rxchef/` below the
current project. Global state lives in the operating system's user configuration
directory. Reads merge both scopes and project entries win on name collisions.

Stored objects include named pipelines, variables, and run history. Commands
that mutate pipelines or variables use project scope unless `--global` is given.
Listing commands accept `--project` or `--global` filters.

The core `rxchef` library and `bake` integration API do not require storage.
This separation is useful for servers and editor plugins: they can execute an
explicit recipe without creating `.rxchef/` or modifying user state.

Do not commit `.rxchef/` blindly when it contains secrets or history. Export a
pipeline explicitly when it is intended to be shared.
