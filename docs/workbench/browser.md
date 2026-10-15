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

Choose **Graph mode**. The inspector offers node selection and explicit
Source/Target/Port connection controls as an alternative to canvas handles.
Create an Input, an Operation and an Output. Connect each producer's `data` port
to the next node's `data` input. Give inputs and outputs distinct, useful names.
Select **Run graph** after binding every input required by the selected outputs.

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

Execution freezes a definition and its input/connection revisions. Any later
edit invalidates that run's output and requests cancellation. A submission that
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

Expand **Project**, name the session, and use **New project graph** or **New
project recipe** to add definitions. **Save project locally** waits for the
IndexedDB transaction to commit. **Open saved project** restores the definitions,
selected items, theme and bounded run metadata. Inputs and results must be rebound
and rerun. **Export project** creates a portable `.rxproject.json` backup.

Local project saves belong to this browser profile and origin. Changing host,
port, profile or client can expose a different local library. Clearing site data
removes that library. Browser storage and CLI/native repositories do not sync.
Export before clearing data; see [portable projects](portable-projects.md).

If storage is denied, full, corrupt or blocked by another tab, the workbench
reports a failure. A successful `put` request alone is not a successful save.
Keep the current page open, export a validated project if possible, close stale
tabs and retry. Refreshing loses in-memory input and connection credentials.
