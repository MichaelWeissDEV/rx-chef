# 0.0.1 verification

The maintainer explicitly accepted the UI and authorized publication after CLI/server verification. The prepared runtime uses version 0.0.1, protocol 2 and portable graph/project format 1.

- Default workspace: 2,525 Rust tests PASS.
- All features: 2,273 integration/operation cases PASS in the initial run; 306 remaining workspace cases and five Rustdoc cases PASS on isolated rechecks. The two test-crate Rustdoc cases complete coverage (2,581 distinct cases; three Rustdoc cases overlap the recheck).
- Web: 107 unit cases PASS; corrected complete 32-case browser suite PASS.
- Optimized CLI/server, command help, JSONL protocol, real HTTP/worker/cancellation tests PASS.
- CLI, TUI, headless and embedded-web archives: install/smoke/uninstall/reinstall PASS on macOS ARM64.
- macOS app/DMG built; DMG integrity PASS. Native installer/offline acceptance is not inferred from builds.
- Local Linux ARM64 Docker web/headless runtime smokes PASS.
- Earlier tag attempt: Linux x64 and ARM64 Docker web/headless builds and runtime/host-network smokes PASS in [run 38812265039](https://github.com/MichaelWeissDEV/rx-chef/actions/runs/38812265039). Native Windows then failed on MD6 DLL-import linkage; this attempt is superseded, with its actual observations retained in `ci-attempt-38812265039.json`.
- Corrected per-package cargo-dist host archives: checksum verification, extracted CLI all-byte decode, TUI help and authenticated server multi-input graph smokes PASS on macOS ARM64 (`precise-archives.json`). These locally built archives are separate from CI downloads; no clean build commit is inferred.

The initial host wrapper remains FAIL: its browser fixture assumed random recipe names sorted after Recipe 1; a concurrent default-feature browser build also replaced the CLI executable during the all-features test. The fixture was corrected, the full browser suite passed, and remaining feature tests were repeated sequentially. No failing attempt was relabeled or removed. Runtime engine implementation was unchanged during these test/tooling corrections. Exact release-tag CI remains mandatory.

CI fixes also select Git Bash instead of WSL on Windows, select native Strawberry Perl for OpenSSL, trust only the mounted /src checkout in the release container, publish core before dependent crates, and avoid requiring unpublished core dependencies during pre-publication package-content checks. Installer inventories exclude earlier versions retained in local build directories.

`checks.json` retains commands and source/dirty metadata; raw logs remain in ignored target/rc-evidence. `artifacts.json` lists hashes and actual acceptance scope. GitHub CI/release status is recorded in the current documentation when known. These host observations do not certify unexecuted OS/architecture combinations.

MD6 static-linkage correction: all six existing vectors/argument checks PASS with default features and again with all features. Native Windows confirmation remains part of the corrected tag gate.
