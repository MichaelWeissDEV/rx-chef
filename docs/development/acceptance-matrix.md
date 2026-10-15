# Workbench acceptance matrix

This is a repeatable test specification. It does not assert that each scenario
has passed on every platform. Actual outcomes belong in the
[RC readiness report](0.2.0-rc.1-release-readiness.md), with commands, environment,
artifact identity and retained logs. Use an isolated profile/account and
synthetic data. Never reuse a debug app as production installer evidence.

## Cross-client scenarios

| ID | Procedure | Required observation | Automated starting point |
|---|---|---|---|
| A Recipe | Decode `SGVsbG8=` with From Base64, then To Hex; copy/save/load/reexecute | Exact representation, preserved step order, explicit rebound input | Web E2E recipe acceptance |
| B Multi-input | Use all-byte payload plus two distinct binary inputs, fan-out and three outputs | Independent outputs match bytes and byte counts; export each | Graph tests, terminal fixture, archive smoke |
| C AES | Bind ciphertext, key and IV separately; decrypt with matching parameters | Exact expected plaintext; encoded text not mistaken for decoded key bytes | Graph AES unit test and web E2E |
| D Branch isolation | Fail an AES/adapter branch beside a valid branch | Failed node and blocked dependants identified; valid output exports exactly | Graph branch regressions |
| E Cancellation | Run CPU-bound work, cancel before/during submit and while running; edit/change connection/exit | Terminal state only after worker termination; no late stale output; no orphan | Job race tests, native worker tests, TUI PTY |
| F Project | Save/export three graphs and two recipes, settings and run history; restart/import | Definitions/settings persist, input/results/token absent; explicit rebind/reexecute | Store acceptance and browser project E2E |
| G Desktop offline | Start installed release app with network disabled; local recipe/graph and native dialogs | No network dependency, exact exported bytes, successful native save/open | Native GUI acceptance required |
| H Remote | Run one fixture through CLI, TUI and web against the same authenticated server | Matching outputs, protocol/capabilities agree, per-client cancellation/policy behavior | Remote integration plus PTY/E2E |
| I Install | Install, launch, run fixture, uninstall, reinstall and rerun on a clean system | Correct executable/architecture, exact output, clean process shutdown | Archive smoke; native installer acceptance separately |

Record per-client outcomes for each scenario. A passing Rust test cannot stand in
for a native file dialog, and a browser download cannot stand in for an OS
installer launch. Clipboard permission failures must remain visible failures or
explicitly untested steps, not silent substitutions.

## Negative and boundary matrix

| Boundary | Cases | Invariant |
|---|---|---|
| Bytes | Empty, every byte value, invalid UTF-8, BOM, embedded NUL, large binary | Exact data survives transport and file export |
| Graph | Cycle, duplicate ID, invalid/duplicate port, missing dependency, detached node, output selection | Validate whole definition; execute only selected dependency closure |
| Arguments | Literal/edge conflict, strict typed adapter, invalid syntax, wrong key/IV representation | Reject invalid data before dispatch, isolate branch errors |
| Budgets | Nodes/edges/depth/definition/input/node/live/output at boundary and beyond | Typed limit failure; no partial output masquerading as success |
| Jobs | Queue full, queued cancellation, delayed ID, transport loss, timeout, expired retention | No fabricated terminal state or stale publication; bounded admission/retention |
| IPC | Oversized/truncated frame, invalid JSON, parent lifeline EOF | Bounded parsing; terminate/reap worker |
| Project | Unknown nested fields, unsupported version, duplicate selection/ref/history, malformed time/duration | No silent migration or replacement of valid saved data |
| Persistence | Failed replace, count/byte quota, thread/process writers, rename collision | Prior valid entries preserved; cooperative quotas serialize |
| Browser storage | Denied access, quota error, abort after request success, corrupt record, tabs | Save acknowledgment only after transaction commit; recoverable visible error |
| Secrets | Disabled steps, embedded recipes, historical terminal controls | Known sensitive literals excluded from portable saves; controls escaped in human terminal output |

## Platform and installer record

For each OS/architecture, retain the native build command, toolchain, package
hash/size, install destination, actual launched executable path, local and
remote fixture results, cancellation/process evidence, GUI dialogs, offline
result, uninstall/reinstall result and signing status. Minimum target rows are
Linux x86_64/aarch64, macOS Intel/ARM64 and Windows x86_64.

Linux address-space limits, macOS sampled physical-footprint enforcement and
Windows Job Objects have different semantics. Record the native resource test
rather than inferring behavior from cross-compilation. Network/volume/container
failures remain infrastructure failures until a complete rerun passes.

## Performance observations

Use fixed 10/50/100-node fixtures and record browser version, hardware, viewport,
render/interaction timings and available memory measurements. Retain raw
attachments. Check that inputs remain editable and cancellation/scroll/selection
stay responsive. Measurements on one machine are observations, not universal
performance claims. Sequential DAG scheduling and logical byte accounting are
documented separately from OS memory and future parallel scheduling.
