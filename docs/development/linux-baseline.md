# Linux release verification

- Commit: `fe367ee2ef145a5745ab27a93fea42d6a38dec09`
- Timestamp: `2026-08-13T22:21:01Z`
- Docker base: `debian:bookworm-slim`
- Distribution: `Debian GNU/Linux 12 (bookworm)`
- Architecture: `x86_64`
- Kernel: `Linux 6.10.14-linuxkit`
- Rust: `rustc 1.96.1 (31fca3adb 2026-06-26)`
- Cargo: `cargo 1.96.1 (356927216 2026-06-26)`
- Default tests: passed (see command list below)
- All-features tests: passed

## Results

| Gate | Result |
|---|---|
| Cargo metadata | pass |
| rustfmt | pass |
| Workspace check | pass |
| Workspace build | pass |
| Workspace tests | pass |
| Clippy correctness/suspicious | pass |
| All-features check | pass |
| All-features tests | pass |

## Known failures

The first image revision failed before workspace compilation because
`fontconfig.pc` was absent. Installing Debian's `libfontconfig1-dev` and
`libfreetype6-dev` resolved that container dependency. No project gate in the
table above remains failing.

This baseline is Linux x86_64 only and makes no macOS, Windows, remote-CI,
publication, or long-running fuzzing claim.
