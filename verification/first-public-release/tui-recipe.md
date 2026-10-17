# Recipe editor PTY validation

Status: **NOT TESTED — awaiting freshly rebuilt release binary**.

Prepared `scripts/tui-recipe-smoke.py` specifically for the real recipe editor,
complementing the existing graph PTY smoke. Syntax validation passed with
`python3 -m py_compile scripts/tui-recipe-smoke.py`. No Cargo/npm commands or
TUI executions were performed while the serialized release gate was running.

Run after release compilation completes:

```sh
python3 scripts/tui-recipe-smoke.py target/release/rxchef_tui \
  --report target/first-public-evidence/tui-recipe-smoke.json
```

The script uses an isolated store and real POSIX PTY with the existing ANSI
screen decoder. It polls observable UI/file/process state against deadlines;
there are no fixed sleeps or mock operation results. Output assertions inspect
the right output pane, excluding input text that could otherwise produce false
positives. JSON evidence records the actual binary SHA-256, saved recipe and
reconstructed screens. It rejects a binary modified during the test.

Scenarios prepared: operation search and two-step recipe creation; argument
editing with Tab; From Hex producing NUL/non-UTF8 bytes followed by To Hex;
intermediate exact HEX inspection and final text navigation; output-changing
argument edits; store save and JSON validation; delete/reload/execution;
pipeline keyboard navigation; help at 20×8 and normal dimensions; normal quit,
restart, saved recipe execution and Ctrl+C with terminal flags restored.

Limitations found in source: recipe mode accepts text input and exposes no
byte-output file export key (`e` edits arguments; `s` saves recipe/history;
`x` changes display). This script therefore cannot certify recipe-mode binary
file import/export. The separate graph PTY smoke exercises exact binary file
binding/export. Native Windows console behavior, installer lifecycle and orphan
worker process detection are outside this recipe-specific POSIX test.
