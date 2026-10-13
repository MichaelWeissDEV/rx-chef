# Product completion roadmap

This is the active product direction from 13 October 2026. It supersedes plans
for a source-code analysis, call-graph, taint-analysis, vulnerability-detector,
or independent binary-analysis platform. Existing CyberChef operations,
including optional disassembly, remain supported. Do not add placeholder crates
for deferred analysis systems.

## Architecture decision (M1)

Keep the existing workspace and public `rxchef` APIs. The inspected checkout has
478 registered operations, a shared control-flow engine, Magic, streaming Scan,
a binary-safe integration API, JSONL/JSON-RPC 2.0 over stdio, CLI, Ratatui TUI,
and an independent Store. There is no FastAPI integration, HTTP server, web
frontend, or desktop application in this checkout.

| Boundary | Responsibility | Dependencies |
|---|---|---|
| `rxchef` / `src/` | Registry, transforms, recipes, Magic, Scan, integration, FFI | No UI framework or server dependency |
| `rxchef-store` | Recipes, projects, variables, history | Serialization and filesystem primitives |
| `rxchef-client` | Shared request/result/error contracts, local and optional HTTP client | Core; optional Reqwest |
| CLI and TUI | Input, output, interaction, local persistence | Core, Store; TUI execution uses Client |
| Planned `rxchef-server` | HTTP/RPC/WS, auth, policy, supervised jobs, server-owned persistence | Client and Store; no private engine |
| Planned `apps/web` | Shared React/TypeScript workbench | Typed transport adapter |
| Planned `apps/desktop` / `rxchef-gui` | Tauri shell, direct local engine, optional remote mode | Same frontend and Client; no mandatory server |

Preserve existing package identities. Add actual implementation crates only
when needed. Browser assets must remain outside Core. A headless server must
build without Node or frontend assets. Keep stdio RPC backward compatible.

## Milestone status

| Milestone | Status in this checkout | Remaining acceptance work |
|---|---|---|
| M1 Architecture | Reviewed; boundaries recorded above | Revisit worker isolation with platform prototypes |
| M2 Client/protocol | Implemented execution/discovery/validation/search/export foundation and bounded trace artifacts; optional HTTP adapter | Magic/Scan, projects/recipes, jobs, full server negotiation; migrate all client calls |
| M3 Server | Implemented & hardened | Axum REST `/api/v1`, HTTP/WS JSON-RPC 2.0, WebSocket connection & in-flight limits, supervised worker processes, cancellation, timeouts, retention, Constant-Time Auth, Scope & Host/Origin policy |
| M4 Design system | In progress | Shared offline frontend, design tokens, light/dark themes, accessible components |
| M5 Web | In progress | Workbench, editors, import/export, exact-byte views, control-flow visualization, undo, palette, browser E2E |
| M6 Desktop | In progress | Tauri direct execution, remote mode, explicit managed server, native file/clipboard/keyring integration, platform smoke tests |
| M7 TUI | Trace correctness and bounded previews fixed; edits no longer implicitly execute | State/render/event separation, asynchronous execution, client discovery/arguments, remote sessions, files/HEX, palette, terminal-size tests |
| M8 CLI | Existing CLI retained | Doctor, connection management, examples; check command consistency and help |
| M9 Operations | Existing 478 operations and verification infrastructure retained | Systematic quality audit, independently verified examples, further Magic/Scan work |
| M10 Audit | Focused contract/regression coverage added | Full security review, fuzzing, differential tests, performance evidence, frontend E2E and platform runs |
| M11 Release | Client inserted into package dependency order; optional remote tests in native gate | Actual server/GUI/web artifacts and all-platform installation/removal evidence |

These are implementation statuses, not completed release gates. The requested
product is **not release-ready**. No release, push, package upload, or tag is
part of this local work.

## Server implementation requirements

The current engine is synchronous. Its output-size check runs after each
operation returns; it cannot bound an operation's temporary allocations or
interrupt a blocking native function. Do not expose unrestricted engine work in
an HTTP handler or claim that dropping a future cancels it.

Prototype one supervised worker process per job, with a bounded admission queue,
bounded protocol output, parent-controlled deadlines, kill-and-reap cancellation,
and supported OS resource controls. Terminate workers when their supervising
server exits. Publish actual supported limits through capabilities. Job IDs,
status, results, errors, and retention must be bounded. Report progress only
where the engine knows it. Test a running CPU-bound operation, not just Sleep.

Default to loopback, including host validation. Network exposure must be
explicit and authenticated, with TLS or a documented trusted TLS termination
boundary. Authenticate REST, RPC, WebSocket upgrades and job access consistently.
Validate Origin and Host independently of bind address. Define WebSocket browser
authentication without putting durable credentials in URLs or browser storage.
Disable network/filesystem/process-effect operations by default; canonicalize
aliases before allowlist decisions. Check nested recipes too. Review the registry
metadata and indirect execution paths before treating effect labels as a sandbox.
Server project identifiers must never become arbitrary host paths. Define one
server-owned storage root and reject path traversal, symlink escapes and races.

## Required acceptance evidence

Track CLI binary-safe execution and recipe persistence; local/remote TUI;
standalone/remote desktop; browser recipe editing; headless REST/RPC without web
assets; exact `00..ff` and invalid UTF-8 across every transport; Fork/Merge,
Subsection, registers and jumps; offline desktop/web; unauthorized and denied
side effects; and independent installation/removal of each component.

Record each scenario as passed, failed, or not tested. A mock HTTP contract test
is not evidence of a working network server. A default macOS workspace test run
is not evidence of Linux, Windows, optional native features, desktop or browser
correctness. Internal round-trip tests do not replace independent operation
reference vectors.

Local build/test evidence and the ten-part status overview are recorded in the
[interface foundation report](interface-foundation-report.md).
