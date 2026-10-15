# Save work locally

Choose the saved artifact according to what you want to reopen. A definition
contains the transformation; a result file contains its exact output. Saving one
does not save the other.

| Surface | Save definitions | Save exact results | Where state lives |
|---|---|---|---|
| Browser | Save recipe, Save graph, Save project locally, Export project | Raw-byte download or named graph output download | Browser profile/origin plus files in the chosen download folder |
| Desktop | Recipe/graph saves, local project library, native project import/export | Native output save dialog | App webview profile plus explicitly selected host files |
| CLI | Recipe exports, graph store/files, portable project exports | `--output`, named graph outputs, or redirected raw stdout | Explicit files and scoped Store |
| TUI | Recipe save and graph `s` | Graph `e` exports the selected output | Scoped Store and explicit file destinations |
| HTTP/Docker server | Validate/execute definitions supplied by clients | Client retrieves and saves result bytes | Bounded in-memory jobs; no persistent project database |

## A useful backup workflow

1. Save your recipe or graph while editing. For multiple definitions, export the
   complete `.rxproject.json` project.
2. Keep the original input files separately. Record byte lengths and hashes when
   exact reproduction matters.
3. Download/export each needed result as raw bytes. A truncated text/HEX preview
   is not a backup of the payload.
4. Reopen the saved definition/project, deliberately bind inputs and execute.
   Compare the output with the saved expected bytes.
5. Keep a copy outside browser/app profile storage before clearing site data,
   removing profiles or reinstalling the application.

Portable projects preserve settings, selected definitions and bounded run
metadata. They intentionally exclude bound input bytes, output payloads and
credentials. File references never cause automatic host-file reads. Known
sensitive literal arguments are rejected; use named inputs for key material.
See [portable formats and recovery](portable-projects.md) for exact limits.

For a recipe saved from the TUI, the CLI can write its exact output to a file:

```sh
rxchef pipeline run SAVED_NAME --input-file input.bin > output.bin
```

Replace `SAVED_NAME` with the saved pipeline name. This reruns the definition
with explicit input; it does not reconstruct bytes from the TUI preview.

## Deterministic CLI storage

Use an absolute `RXCHEF_HOME` to choose the global Store in automation:

```sh
export RXCHEF_HOME="$PWD/local-rxchef-state"
rxchef pipeline list --global
rxchef graph store --help
```

`rxchef project init` creates a project-local `.rxchef` Store marker; commands in
that directory or descendants discover it. Use explicit scope flags when a job
must not depend on the caller's current directory. Graph/project repositories
have cooperative locks and quotas; legacy variable/recipe read-modify-write
behavior has different concurrency limits. [Storage](../concepts/storage.md)
explains paths, permissions, secret classification and recovery.

## Reconnect and restart

Reloading a browser page or restarting the desktop app clears session connection
credentials. Enter the token again. Restored definitions have unbound inputs and
no old results. A browser origin change (scheme, hostname or port) can show a
different local library. An HTTP server or Docker container restart does not
synchronize that library with another client.
