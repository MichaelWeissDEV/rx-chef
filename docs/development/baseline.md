# Development baseline

This baseline records the unmodified checkout before release consolidation work.
It reports command results observed locally; it does not claim a remote CI state.

## Environment

| Field | Observed value |
| --- | --- |
| Commit | `112a61d9e5f559405c0804bbea91e5bb2582c8f6` |
| Date | 2026-08-13 |
| Rust | `rustc 1.96.1 (31fca3adb 2026-06-26) (Homebrew)` |
| Cargo | `cargo 1.96.1 (356927216 2026-06-26) (Homebrew)` |
| OS | Darwin 25.4.0 |
| Architecture | arm64 |
| Workspace version reported by Cargo metadata | `0.0.1` |

The worktree was clean before the baseline commands. `cargo metadata
--format-version 1` completed successfully. The normal build did not change a
tracked file.

## Quality gates

| Gate | Command | Result |
| --- | --- | --- |
| Format | `cargo fmt --all --check` | Pass |
| Check | `cargo check --workspace --all-targets` | Pass |
| Build | `cargo build --workspace --all-targets` | Pass |
| Tests | `cargo test --workspace` | Pass |
| Clippy | `cargo clippy --workspace --all-targets -- -D clippy::correctness -D clippy::suspicious` | Pass with non-denied warnings |
| All features | `cargo check --workspace --all-targets --all-features` | Pass |

The operation integration suite reported 1,742 passed and 7 ignored tests. The
workspace also ran pipeline, core, CLI, store, TUI, and documentation tests.
The successful exit status is the authoritative result; this file deliberately
does not present the count as a permanent project-wide test claim.

## Known compiler errors

None in the commands above.

## Known test failures

None in the commands above. Seven operation tests were ignored at baseline and
must be audited rather than treated as release coverage.

## Known Clippy findings

There were no `clippy::correctness` or `clippy::suspicious` failures. The run
reported numerous warnings from other lint groups across core code and tests,
including `precedence`, `manual_is_multiple_of`, `needless_borrow`,
`manual_strip`, `needless_range_loop`, `len_zero`, and weak test patterns.
These warnings are not represented as a clean `-D warnings` gate.

## Mandatory initial audit findings

The requested source audit found the following release-relevant issues:

- PGP encrypt, decrypt, verify, encrypt-and-sign, decrypt-and-verify, and key
  generation implementations explicitly describe themselves as stubs.
- Three HASSH client fingerprint tests assert `is_ok() || is_err()`, which proves
  no behavior.
- Several test names or comments explicitly say placeholder or dummy, including
  ECDSA key generation, public-key extraction, RC6 decryption, CSR parsing, and
  image dithering tests. Each requires a semantic review; textual matches used
  as real formatting sentinels or domain terminology are not automatically
  defects.

These findings existed on the baseline commit and were not hidden or counted as
completed implementations.
