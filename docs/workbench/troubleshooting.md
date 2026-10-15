# Workbench troubleshooting

| Symptom | Check | Recovery |
|---|---|---|
| Page opens, operations do not load | Missing/expired session token, server reachable, protocol version | Reconnect with the current token; reload requires authentication again |
| Headless server returns no page | Server built without `web-ui` | Use the embedded-web build for a browser workbench |
| Correct token still denied | Host/Origin policy, token scope, execution allowlist | Correct the configured deployment policy and use the intended scope |
| Graph has no usable output | Required named bindings, connection ports, node errors | Inspect dependency errors, bind decoded bytes, run explicitly |
| AES key length or encoding error | HEX/Base64 text versus decoded bytes; mode/IV/padding | Decode textual input before a byte argument port and match the cipher settings |
| Empty output after restore | Definitions restored without execution data | Rebind inputs and rerun; a saved project is not a result cache |
| Old output disappears after edit | Revision invalidation | Finish editing and execute the current definition |
| Cancellation stays pending | Worker not terminal yet or cancel transport failed | Inspect the error and continue observing; wait for confirmed termination |
| One branch fails, another succeeds | Per-output status rather than only HTTP status | Export only a successful named result; resolve the failed dependency |
| Project absent in another client | Profile/origin/client has a separate library | Transfer a validated portable export |
| Local save fails | Quota/permissions, transaction abort, blocked database upgrade | Export current data, close stale tabs and retry without deleting the prior save |
| Imported file rejected | Wrong format/version, unknown fields, limits or sensitive literals | Preserve the original; inspect validation and use a compatible definition |
| CLI output differs from displayed text | Preview representation or newline conversion | Compare binary files; do not reconstruct data from truncated previews |
| Installer built but cannot launch | Wrong executable, architecture, native dependencies or signing | Record the actual bundle path and OS error; consult the current readiness report |

## Record a useful reproduction

Use synthetic non-secret bytes and the smallest definition reproducing the
problem. Record version, OS/architecture, local/remote mode, build features,
exact command or UI actions, expected bytes, actual byte length/hash, output
status and typed error. For job problems include the job ID, state sequence and
whether the connection or definition changed. For persistence, distinguish the
save acknowledgment, reload, import and subsequent execution.

Redact tokens, personal paths and argument values before sharing logs. A HEX or
Base64 representation still contains the original data. Keep failures and the
successful rerun as separate evidence; an unexecuted platform remains untested.
See the [testing handbook](../development/testing.md) for repeatable commands.
