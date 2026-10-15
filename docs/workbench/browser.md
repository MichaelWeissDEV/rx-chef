# Browser workbench

The browser is a client of the Rust engine. Build the embedded-web server from
the repository root, then open its loopback address:

```sh
npm --prefix apps/web ci
npm --prefix apps/web run build
cargo build --locked --release -p rxchef-server --features web-ui
target/release/rxchef-server
```

Open `http://127.0.0.1:8787`, choose **Set Token**, and enter the token printed by
the server. The token stays in the current page's memory; reloading requires
connecting again. The public page shell does not grant execution access. A
headless build exposes the API without the workbench assets.

## First recipe: decode and inspect

1. Choose **List mode** and remove unwanted starter steps.
2. Add **From Base64**, followed by **To Hex**, using the operation browser.
3. Select text input and enter `SGVsbG8=`. This is encoded text: selecting Base64
   input would decode it before the recipe and change the meaning of the example.
4. Execute explicitly with **Bake**. The result represents the bytes of `Hello`
   as hexadecimal. Spaces and letter case follow the To Hex arguments.
5. Inspect the output views. Copying a textual representation and downloading
   the binary result serve different purposes: choose the action matching the
   format required by the next consumer.
6. Save the recipe, reload it, bind input again, and execute. Saving a definition
   does not preserve the input or prove that a later run has succeeded.

The [operation catalog](../operations/index.md) describes argument defaults and
optional native backends. An unavailable backend is an execution error, not an
empty successful result. Disabled steps remain definitions but do not execute.

## Build and run a graph

Choose **Graph mode**. The starter graph already connects an Input to an
Output. Select the Input and enter data in the inspector on the right. Find an
operation on the left and click it: **Connect new operations automatically**
inserts it into the selected data chain. Selecting an Output inserts before that
output and preserves other branches. Dragging an operation onto the canvas adds
it separately; wire it deliberately before running.

The three-step guide shows missing bindings and disconnected outputs. Click a
readiness issue to select the relevant node. **Inputs ready · run to validate**
means the local readiness check passed; Rust still validates types, arguments
and graph structure when you run. Use **Run all outputs** to choose all or
individual outputs, then **Run graph**. Results appear in named tabs below.

Use **More node types** for combine, typed argument and embedded recipe nodes.
Expand **Connect nodes manually** for Source/Target/Port controls as an
alternative to canvas handles. Give inputs and outputs distinct, useful names.
**Arrange nodes** fits the graph after arranging its dependency layers.

Input modes describe the representation being entered:

| Input mode | Meaning |
|---|---|
| Text | UTF-8 bytes of the entered characters |
| HEX | Strictly parsed hexadecimal bytes |
| Base64 | Decoded binary bytes |
| File | The selected file's bytes, including NUL and invalid UTF-8 |

For three independent sources, open the repository's
`examples/graphs/three-inputs.rxgraph.json` with the graph file control and bind
`payload`, `second` and `third`. The `combined`, `encoded` and `original` output
tabs are separate results. Download each successful result independently.
Bounded previews do not truncate the bytes used for downloads.

For AES, load `examples/graphs/aes-decrypt.rxgraph.json`. Bind ciphertext, key
and IV separately. Connect decoded key/IV bytes to the registry's argument ports;
a file containing hexadecimal text needs **From Hex** before that connection.
Match mode, padding and encoding arguments to the producer of the ciphertext.
A display preview of a key is not an argument binding.

See [graph semantics](../concepts/graphs.md) for combine ports, typed argument
adapters, embedded recipes, dependency selection and resource budgets.

## Editing, cancellation and errors

Execution freezes a definition and its input/connection revisions. A later
execution-affecting edit invalidates that run's output and requests cancellation.
Moving nodes, arranging them and changing display labels preserve results. A submission that
has not yet returned its job ID is cancelled when the ID arrives. A transport
acknowledgment is not proof of worker termination: the client continues observing
the job until its terminal state is known. Cancellation transport failures are
shown and old-revision results cannot replace current output.

Metadata reports actual node state, elapsed time and byte sizes. Unknown values
remain pending. A failed branch blocks its dependants; an unrelated successful
output remains exportable. Inspect both the failed node and each named output
before treating the graph as successful.

**Arrange** lays out DAG layers. Canvas pan/zoom and fit-to-view change only the
presentation. Ctrl/Cmd+Z undoes, Shift+Ctrl/Cmd+Z redoes, Ctrl/Cmd+C/V copies or
pastes graph selections, and Ctrl/Cmd+Enter runs. Text fields retain their normal
editing shortcuts. Copying selected nodes includes their internal connections.

## Projects and local persistence

Expand **Project files & library**, name the session, and use **New project
graph** or **New project recipe** to add definitions. The project strip keeps
Save and Export visible and indicates unsaved definition changes. **Save project locally** waits for the
IndexedDB transaction to commit. **Open saved project** restores the definitions,
selected items, theme and bounded run metadata. Inputs and results must be rebound
and rerun. **Export project** opens a dialog: choose individual graphs and recipes, layout,
appearance/selection, run history and external references. **Team / Git** omits
personal settings, history and references by default. **Complete definitions**
selects all optional metadata; input/output bytes and credentials are excluded
in either profile. Preview the validated JSON, then **Download project file**.

**Import project** validates the file and shows an import review. Confirm
**Open imported project** to replace the current workspace; Cancel keeps it.
Save or export your current work before replacement. Inputs must be rebound.
The same file works in the desktop app and the server's browser workbench;
see [portable projects](portable-projects.md) for sharing through Git.

Local project saves belong to this browser profile and origin. Changing host,
port, profile or client can expose a different local library. Clearing site data
removes that library. Browser storage and CLI/native repositories do not sync.
Export before clearing data; see [portable projects](portable-projects.md).

If storage is denied, full, corrupt or blocked by another tab, the workbench
reports a failure. A successful `put` request alone is not a successful save.
Keep the current page open, export a validated project if possible, close stale
tabs and retry. Refreshing loses in-memory input and connection credentials.

## Compact panels, Auto-Bake and flow reports

The mode switch, project save/export and project library share one workspace
bar. **Graph tools & connections** contains node creation, graph import,
selection/copy, undo/redo, output selection, guidance and manual connections.
The closed menu leaves the canvas available for editing.

In **View**, toggle the operations and inspector sidebars, choose rounded,
square or flowing edges, and enable **Auto-align nodes** for ongoing dependency
layout. **Arrange nodes** performs a single arrangement. These presentation
changes preserve results.

**Auto-Bake graph** and **Auto-Bake recipe** are off initially. When enabled,
input/definition edits wait 500 ms before execution. Each revision is submitted
once, and another worker is allowed to finish/cancel before the next revision
starts. Graph Auto-Bake waits for required inputs and outputs. It does not run
from layout edits or while Graph mode is hidden. Disable Auto-Bake to return to
explicit execution. Existing resource limits and cancellation still apply.

Both result areas offer **Collapse**, **Undock**, and **Dock**. An undocked panel
is a movable, resizable window inside the current app page; it does not open a
second OS application or transfer data to another service. The docked graph
result area can also be resized vertically. Named graph outputs use tabs;
**Compare graph outputs** lays them out side by side. Hide individual outputs
and restore them with **Show all graph outputs**.

In List mode, **Keep recipe result** retains a completed result as a numbered
tab for comparison with later runs. Up to six results / 16 MiB can be retained
in memory; they are excluded from project export and disappear when leaving
List mode or reloading. Remove snapshots when no longer needed. Operation and
input panels can be hidden from the List toolbar. **Recipe files & tools**
contains recipe save/load and undo/redo; **Transformation trace** opens the
existing bounded actual step-output trace after execution.

**Transformation steps** in Graph mode opens a readable dependency list without
exporting. It explains node operations, parameters/defaults, connections,
embedded recipe order and available status/timing/output-size metadata.
Independent branches are listed in dependency order, not chronological execution
order. Graph execution does not capture intermediate payloads.

**Export data flow SVG** saves a standalone scalable image containing all graph
nodes, edges and the transformation list, using the current layout and edge
style. Input/output payloads are omitted and sensitive argument values are
redacted. Review ordinary literals before sharing. Image fields over 1000
characters are shortened; the interactive list and project definition remain
the sources for full ordinary values. A draft diagram or a description of a
step is not proof that it executed successfully.
