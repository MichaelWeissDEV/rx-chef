# First public release readiness

Assessment updated: 2026-10-18, Europe/Berlin. Candidate: `0.2.0-rc.1`.
Remote `master` at assessment: `58bf552acc359a505648eba3fb3495bbfb2fe67a`.
Local CI-fix commit: `cfd0a8f1` on `codex/release-ci-fixes`; not pushed.
**NOT READY FOR RELEASE**. No push, tag, GitHub Release, registry publication,
remote repository edit, workflow dispatch or artifact upload was performed.

This report supersedes the historical 0.0.1 and earlier RC assessments. A local
working-tree build is not a green immutable release commit. Full command logs
and structured records are retained under `target/first-public-evidence/`;
portable summaries are in `verification/first-public-release/`.

## Source, version and tools

The checkout initially matched the requested failing CI commit and was clean.
The maintainer subsequently pushed `58bf552a`, which was the basis of the first
native CI run. The local fix commit `cfd0a8f1` has no native runner result yet.
Read-only GitHub checks found tags `v0.0.1` and `v0.1.0`, and no GitHub Release.
All five crates.io package API requests returned 404. The Homebrew tap contents
request returned 404; absence and insufficient access cannot be distinguished.
Neither existing tag was modified. Version availability must be checked again
immediately before publication.

Cargo workspace and internal dependencies, both Cargo locks, web manifests,
Tauri, contract fixture, generated operation status, README and changelog now
agree on `0.2.0-rc.1`. The workbench displays the package version dynamically.
Arch uses `0.2.0rc.1`, retaining `_releasever=0.2.0-rc.1` for the source tag.
The checked-in AUR checksum is deliberately an invalid placeholder until a real
source archive exists; it must never be used as evidence of source verification.

Host: macOS ARM64, Rust/Cargo 1.97.1, Node 26.7.0, Python 3.10,
cargo-dist 0.32.0, cargo-audit 0.22.2. CI uses Rust 1.96.1 and Node 22.20.0;
this host cannot certify those other native runner environments.
`verification/first-public-release/source-files.json` fingerprints 1,272 local
non-documentation source files. Final artifact checksums identify actual files,
not a claimed clean build commit.

## Fixes and failure classification

| Problem | Classification | Change and evidence boundary |
|---|---|---|
| GNU tar reports directory changed during archive creation | Packaging code error | Archive is a sibling of the content staging directory; explicit members, no AppleDouble extras, streaming SHA-256, atomic hardlink publication, no-overwrite checks and failure cleanup |
| Windows macro doctests report 152 unresolved CRT symbols | Link configuration | Tauri 2.7.2 creates an empty `msvcrt.lib`; Cargo passes its search directory into unrelated rustdoc links. Disable Tauri's shim and consistently select Rust static CRT. Both macro examples now execute through a test harness. Native Windows rerun remains required |
| ARM64 queue ordering assumes spawn order | Test flakiness | Observe first job Running before submitting the queued job; bounded polling replaces fixed startup sleeps. 25 server tests and 40 loaded queue repetitions passed |
| Old version in distribution smoke and workbench header | Release configuration | Derive smoke version from locked metadata; derive UI version from package.json; refresh versioned contract fixture |
| Homebrew token environment and broad release permissions | Release configuration | Homebrew and release host use environment `release`; write permission is explicit on the host job, default is read-only; obsolete remote cleanup workflows removed |
| Masked package/style checks and incomplete package smoke | Release configuration | Lint errors fail; Python installed explicitly where used; native/dry-run DEB/RPM/AUR checks now include installed TUI PTY and reinstall tests. Configured checks are not executed-platform passes |
| Registry API visibility confused with dependency resolvability | Publishing code error | Fail closed on ambiguous requests; wait for exact, non-yanked sparse-index entry; dry-run always calls Cargo; allowlist package names/arguments |
| Dependency advisories | Dependency risk | Minimal updates reduce workspace advisory matches 27→11; mandatory RustSec and npm audit gates added. No advisories suppressed |

Detailed evidence lives in the repository at
`verification/first-public-release/ci-windows.md`, `packaging.md`, `server.md`
and `dependencies.md` (these files are outside the published MkDocs tree).

## Current test evidence

The serialized host rehearsal finished with two failed gates: the mandatory
RustSec audit (11 matches) and an ESM import error in a newly added browser test.
The browser test was corrected and 32 tests passed against the unpacked release
web-server and CLI archives. The current local CI-fix commit passed a subsequent
complete Rust matrix; the failed host wrapper is retained as FAIL.

The [native platform run on `58bf552a`](https://github.com/MichaelWeissDEV/rx-chef/actions/runs/39056219136)
failed on all five native runners. Linux/macOS and the Linux release container
hit a differential-test assertion that disallowed expected invalid-input
rejections from non-deterministic PGP operations. Native Windows failed earlier
in the packaging regression: its Windows PATH and `C:` archive paths were passed
to Git Bash/tar. Commit `cfd0a8f1` fixes both causes locally; the Windows path
fix requires a new native run. The Windows GNU cross-build job passed.

| Check | Status | Scope |
|---|---|---|
| Version/publication/acceptance rules | PASS | 13 version, 10 publishing, 5 UI acceptance and public-download regression tests |
| Packaging regressions | PASS locally | 12 tests on macOS, including Windows path translation; GNU tar 1.35 and BSD tar passed the earlier 11 archive tests; native Git Bash rerun pending |
| Server integration and concurrency repetitions | PASS | 25 tests plus 40 queue repetitions across four simultaneous processes |
| Frontend unit/typecheck/lint | PASS | 107 unit tests and clean checks on updated dependencies/version |
| Default Rust workspace | PASS locally | 2,526 tests, zero failures/ignored, including both executable macro doctests |
| All-features Rust workspace | PASS locally | 2,582 tests, zero failures/ignored; both feature-set Clippy checks and rustfmt passed |
| Corrected real-server browser suite | PASS locally | 32 tests against unpacked release web-server and CLI archives |
| npm dependency audit | PASS | No known vulnerabilities at audit time |
| RustSec dependency audit | FAIL | 11 workspace advisory/version matches; one RSA match in fuzz lock |
| actionlint / dist generated workflow check | PASS | Local syntax and generated-surface validation, no CI dispatch |
| Strict MkDocs build | PASS | Repeated during documentation updates; final report build recorded separately |
| UI publication acceptance | FAIL | Deliberately pending for the new version; prior acceptance does not carry forward |

The first host attempt remains FAIL in the record. Versioned contract regeneration
was initially missed. Concurrent Cargo builds also replaced the root crate's
unhashed rlib during rustdoc and caused E0460; the final rehearsal is serialized.
Editing an executing Bash gate changed its input offsets, so that attempt was
stopped rather than relabeled successful. The version-header browser test then
needed an ESM-safe JSON read; its corrected real-server rerun is recorded.
No tests were disabled, no `continue-on-error` was introduced, and no failure was
turned into PASS by changing the release conditions.

Representative executed commands (the gate source and retained full log contain
all additional checks and feature combinations):

```sh
bash scripts/release-check-linux.sh --host
cargo test --locked --workspace
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D clippy::correctness -D clippy::suspicious
npm --prefix apps/web run typecheck
npm --prefix apps/web run lint
npm --prefix apps/web test
# The corrected ESM test was rerun without a concurrent Cargo build:
(cd apps/web && npx playwright test)
python3 scripts/test-distributions.py
python3 scripts/test-publish-crate.py
python3 scripts/test-release-download.py
mkdocs build --strict
actionlint
dist generate --check
```

## Platform and artifact matrix

| Platform | Source/runtime | Archive/package install/reinstall | Desktop production acceptance |
|---|---|---|---|
| macOS ARM64 | PASS locally for default/all-features Rust tests | PASS locally for four archives, checksum and install/uninstall/reinstall; archive source predates local CI-fix commit | NOT TESTED to publication standard |
| macOS Intel | NOT TESTED on changed source | NOT TESTED | NOT TESTED |
| Linux x86_64 | NOT TESTED on changed source | NOT TESTED | NOT TESTED |
| Linux ARM64 | NOT TESTED on changed source | NOT TESTED | NOT TESTED |
| Windows x86_64 MSVC | NOT TESTED on changed source | NOT TESTED | NOT TESTED |

The native CI run covers `58bf552a`, not local `cfd0a8f1`. GNU cross
compilation is not an MSVC runtime test. A source build is not an installer test.
The requested Linux archives/DEB/RPM/AppImage, Intel/Apple Silicon APP/DMG,
Windows EXE/MSI/NSIS, both server variants and container architectures are only
certified when their actual files pass installation and functional acceptance.
No untested platform is advertised as release-supported.

Docker Desktop was launched in the background, but the local daemon remained
unreachable. The [server container build/smoke run](https://github.com/MichaelWeissDEV/rx-chef/actions/runs/39056219167)
passed on `58bf552a`; local Docker/Compose runtime, Arch makepkg, DEB/RPM and both container
architectures remain NOT TESTED locally. Compose configuration validates;
configuration inspection does not replace runtime evidence. Container bases use
version tags, not immutable image digests; capture reviewed digests before a
reproducible public image build. No registry image was published.

## Crates and package-manager preparation

All five `cargo package --locked --allow-dirty --list` checks pass. Every payload
contains source, README, Apache LICENSE, normalized/original manifest and lock.
Dependent crate license text is included without removing SPDX metadata.
Publishing order is core → Store → Client → CLI → TUI, with bounded index polling
between steps. The release host verifies downloaded public assets against the exact
upload inputs; actual registry CLI/TUI installations must pass before the Homebrew
update is eligible. These external checks are prepared, not executed. The four dependent crates cannot be registry-backed dry-run
verified before their unpublished dependencies exist. Package listings are not
substitutes for compilation of the actual packaged crate.

Homebrew tests resolve the installed keg, exercise CLI transformations or a real
TUI PTY, and uninstall after the check. These regression fixtures pass; a real
public formula installation has not passed. AUR metadata and workflows are
prepared, but no real Arch build or AUR publication is claimed. Homebrew/crates
publishing remains disabled for prerelease tags. AUR and GHCR have no publishing
step enabled and require separate authorization and credentials.

## Open release blockers

1. Resolve or independently assess the remaining RSA, optional YARA/Wasmtime and
   upstream ZIP8 advisory paths. The mandatory audit currently fails. Reachability
   counterevidence is documented; it is not an exploit-validation result or waiver.
2. Obtain all required native CI results on one immutable final commit, including
   Windows default and all-features doctests. No remote workflow was dispatched.
3. Test actual production installers, offline desktop execution, native dialogs,
   byte-exact exports, restart, cancellation, clean process shutdown, uninstall
   and reinstall on every claimed platform. macOS signing/notarization and
   Windows signing/SmartScreen remain unverified; do not describe unsigned files
   as signed or notarized.
4. Complete real Docker/Compose web/headless AMD64/ARM64 and Arch/DEB/RPM tests,
   source checksums, package dependencies and immutable image digest review.
5. Complete registry-backed package dry-runs/bootstrap and real Homebrew tap
   review. Environment protections, secret placement, repository/token rights,
   crates ownership and Trusted Publishing configuration cannot be certified
   from workflow YAML or the restricted read-only adapter.
6. Record explicit maintainer UI acceptance for the final version and immutable
   reviewed source; the validator also rejects uncommitted UI/Cargo changes.

## Publication only after approval

Do not tag or publish this working tree. After all blockers are closed, review
and commit the exact tested source, rerun the full read-only dry-run workflow,
review its complete artifact inventory and obtain explicit maintainer approval.
Recheck version availability; preserve old tags. A stable `0.2.0` needs its own
synchronized version, artifacts and acceptance rather than renaming RC files.

Configure protected `release` environment and allowed tag creators, scoped
crates.io bootstrap token and Homebrew tap token. After first crate publication,
configure each supported Trusted Publisher and remove the bootstrap token.
Signing identities/notarization must be configured or their absence disclosed.
Only then create the new immutable signed tag and authorize the release pipeline.
After each external publication, download and hash the actual public artifacts
and exercise public installation paths; stop follow-up publication on failure.
AUR/registry uploads remain separately gated by approval and setup.
