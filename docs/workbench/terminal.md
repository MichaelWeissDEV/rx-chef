# Terminal graph workflow

The CLI is suitable for automation and exact file output. The TUI adds an
interactive inspector over the same engine. Example commands below run from the
repository root with locally built binaries on PATH.

## Reproducible binary fixture

Create three files without relying on shell encoding or newline behavior:

<!-- rxchef-example: terminal-roundtrip -->
```sh
python3 - <<'PYCODE'
from pathlib import Path
Path('payload.bin').write_bytes(bytes(range(256)))
Path('second.bin').write_bytes(bytes([0, 255, 192]))
Path('third.bin').write_bytes(b'last\x00')
PYCODE
rxchef graph validate --graph examples/graphs/three-inputs.rxgraph.json
rxchef graph run --graph examples/graphs/three-inputs.rxgraph.json \
  --input payload=payload.bin --input second=second.bin --input third=third.bin \
  --output combined=combined.bin --output encoded=encoded.bin --output original=original.bin
python3 - <<'PYCODE'
import base64
from pathlib import Path
source = Path('payload.bin').read_bytes()
assert Path('original.bin').read_bytes() == source
assert Path('encoded.bin').read_bytes() == base64.b64encode(source)
assert Path('combined.bin').read_bytes() == source + bytes([0, 255, 192]) + b'last\x00'
PYCODE
```

A nonzero exit means at least one requested output failed. Inspect per-output
errors even if another output file was produced. With one selected output, stdout
can carry exact bytes:

```sh
rxchef graph run --graph examples/graphs/three-inputs.rxgraph.json \
  --input payload=payload.bin --select original > selected-original.bin
```

Only that output's dependencies require bindings. `--json` is a structured
result envelope rather than raw output bytes. Diagnostics go to stderr. Do not
merge stderr into a binary output file.

## TUI keys

```sh
rxchef_tui --graph examples/graphs/three-inputs.rxgraph.json
```

The following keys apply to the graph screen when no text prompt is active:

| Key | Action |
|---|---|
| Tab | Switch between recipe and graph screens |
| Up/Down | Select a node |
| `l` | Load a graph file or list JSON files in a chosen directory |
| `i` / `h` / `b` / `f` | Bind the selected Input as text / HEX / Base64 / file |
| `r` | Run explicitly |
| `c` | Request cancellation |
| `o` | Select the next output |
| `e` | Export the selected output's exact bytes |
| `s` | Save the graph definition |
| `q` / Ctrl+C | Exit and clean up active work |

Enter confirms prompts; Escape cancels them. Binding a new input invalidates
older results. The inspector reports dependencies, errors, node states and
measured sizes/times. Its bounded HEX preview is never the source for export.
An active job must be cancelled before loading another graph.

Local and remote execution run off the terminal event loop. Terminal resizing
must remain responsive. Exit restores raw-mode and alternate-screen state and
cancels active work; the [PTY acceptance script](../development/testing.md)
checks these behaviors with the real release binary.

## Remote execution and history

Supply `--server URL` to the CLI graph command or the TUI, and set `RXCHEF_TOKEN`
in the invoking environment. The selected input bytes travel to the server;
local file paths are resolved by the client. The server does not open those paths.
Keep tokens out of saved commands and shared logs.

Graph execution can opt into metadata-only CLI history with `--history`.
Project history contains status and sizes, not replayable input. Use explicit
fixture files for reproduction. [Legacy recipe history](../cli/history.md) has a
separate preview model, so treat existing history files as potentially sensitive.
