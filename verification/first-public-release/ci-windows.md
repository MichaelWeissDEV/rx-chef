# CI and Windows stabilization evidence

Status: **NOT READY FOR RELEASE**. Local checks validate the changes below;
no native Windows execution or new same-commit GitHub matrix has occurred.

- Date: 2026-10-17 (Europe/Berlin).
- Base commit: `d07213d86e0849239cb169489221fea5ff7d8411`.
- Working version: `0.2.0-rc.1`; evidence applies to the modified working tree,
  not an immutable release commit.
- Local machine: macOS / ARM64; Homebrew rustc/cargo 1.97.1; cargo-dist 0.32.0;
  actionlint 1.7.12. Native CI selects Rust 1.96.1.
- No remote mutations, workflow dispatches, tags, uploads or publications made.

## Observed CI failure

Read-only GitHub job metadata and Windows logs from
[run 39037928828](https://github.com/MichaelWeissDEV/rx-chef/actions/runs/39037928828)
show:

| Job | Observed result on the base commit |
| --- | --- |
| Windows GNU cross-build container | PASS |
| Native macos-15 / macos-15-intel | PASS |
| Native ubuntu-24.04 / ubuntu-24.04-arm | PASS |
| Linux release container | FAIL (distribution archive self-inclusion; fixed separately) |
| Native windows-latest | FAIL (two macro doctest links) |

Windows runner: Windows Server 2025, `windows-2025-vs2026` image
`20261007.264.1`, MSVC linker 14.51.36231, Rust 1.96.1,
`x86_64-pc-windows-msvc`. Strawberry Perl setup succeeds. Runtime test suites
succeed, including 2167 integration tests. The failure occurs only while linking
`op_test` and `recipe_test` documentation tests: `LNK1120`, 152 unresolved CRT
symbols (including allocation, stdio, floating point, and C++ unwind symbols).
Windows desktop installer build consequently did not execute.

## Root cause and correction

The locked `tauri-build` 2.7.2 implementation creates an intentionally empty
`OUT_DIR/msvcrt.lib` for its custom static runtime mode. It emits that directory
as `cargo:rustc-link-search=native`, and package-local replacement CRT linker
arguments. Cargo's workspace rustdoc invocation includes native search paths
from the GUI package as well: the failed command visibly contains
`/LIBPATH:.../build/rxchef-gui-915c2ae805c09c33/out` and
`/defaultlib:msvcrt`, without Tauri's replacement CRT arguments. This allows the
empty shim to shadow the genuine MSVC import library in unrelated doctests.
This is workspace native-library search-path contamination, rather than an
OpenSSL source failure or evidence that the runtime tests were broken.

Evidence: locked dependency sources `tauri-build-2.7.2/src/static_vcruntime.rs`
(`override_msvcrt_lib`) and `src/lib.rs` (`should_static_link_vc_runtime`), plus
Windows job 117411404036's rustdoc link command. Upstream source:
[Tauri static runtime implementation](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-build/src/static_vcruntime.rs).

Corrections:

1. `.cargo/config.toml` selects Rust's `+crt-static` for the supported Windows
   MSVC target. Rust and native dependency build scripts now receive one
   consistent CRT choice, including vendored OpenSSL.
2. Desktop `build.rs` explicitly disables Tauri's manual static runtime shim.
   Rust continues to select the static CRT; no dummy `msvcrt.lib` is generated.
3. Both macro documentation examples use `rust,test_harness`. Their generated
   `#[test]` functions now execute, instead of merely being compiled inside a
   normal rustdoc binary. No doctests were disabled or ignored.

Native Windows verification remains **NOT TESTED**, including both required
commands `cargo test --locked --workspace` and
`cargo test --locked --workspace --all-features`. A clean native runner is
required to certify the remedy and ensure no old Tauri build output remains.
The native portability script now requires both default and all-feature
workspace tests, plus all-feature check/clippy. Optional dependency provisioning
was added to the native workflow; its Windows vcpkg setup remains NOT TESTED.
No target is declared release-supported solely from this static diagnosis.

## Workflow audit

| Workflow | Classification / result |
| --- | --- |
| `platform-checks.yml` | Existing Linux archive code failure and native Windows CRT configuration failure; full matrix still required on final commit. Native script now runs packaging regression fixtures on each platform. |
| `release.yml` | Release configuration: default token reduced to read; release host explicitly gets contents write and release environment; Homebrew now uses release environment for its secret. Required failed/cancelled/skipped jobs still block hosting. Plan uses pure dist plan. Removed Homebrew lint failure masking. |
| `release-gates.yml` | Reuses complete platform and container gates; no bypass added. |
| `release-dry-run.yml` | Corrected AUR prerelease/source directory rendering, fail-on-HTTP-error checksum fetch, explicit Bash pipefail and Python dependency, strict namcap, Ruby syntax validation replaces a masked gem install. Retention remains opt-in. |
| `build-workbench.yml` | No new source defect established; server archives and desktop bundles are built, but actual desktop install/uninstall/offline UI acceptance is still separate and NOT TESTED here. |
| `build-deb.yml` | Existing install/functional/remove checks; native Linux package execution NOT TESTED locally. |
| `build-rpm.yml` | Runner dependency configuration: install Python explicitly for JSON functional validation. Native Fedora execution NOT TESTED locally. |
| `build-aur.yml` | Release configuration: convert SemVer prerelease to Arch pkgver while preserving source tag, fail checksum HTTP errors, enforce Bash pipefail, remove namcap failure masking, assert CLI transformation result. Native Arch execution NOT TESTED locally. |
| `container.yml` | Existing native amd64/arm64 headless/web build and runtime smoke matrix, no registry publishing. Docker unavailable for this audit. |
| `publish-crates.yml` | Existing release environment, OIDC grant, stable tag/plan guard and topological publication order retained. Helper audit is recorded separately. No publication attempted. |
| `validate-tag.yml` | Existing version/history/UI acceptance gate retained. Pending maintainer acceptance intentionally blocks tag publication. |
| One-off quarantine/cancellation workflows | Removed obsolete master-push workflows that modified historical remote releases/Actions; they are not part of ongoing product validation. |

Official cargo-dist 0.32.0
[hosting source](https://raw.githubusercontent.com/axodotdev/cargo-dist/v0.32.0/cargo-dist/src/host.rs)
shows GitHub `host --steps=create` only computes hosting metadata. The previous
plan must not be described as proven pre-gate remote draft creation. Actual
GitHub publication is the explicit post-gate `gh release create` step.

Repository/environment protection rules, organization policy, secret presence,
secret scopes, PAT tap permissions and crates.io Trusted Publishing identity
cannot be proven from local YAML or the read-only job log. They remain setup
checks before publication. No credentials were inspected or printed.

## Executed checks

| Check | Result |
| --- | --- |
| `actionlint` | PASS |
| `dist generate --check` | PASS (CI is intentionally maintained under allow-dirty=ci) |
| `python3 scripts/test-release-version.py` | PASS, 12 tests |
| `python3 scripts/test-distributions.py` | PASS, 11 tests using native BSD tar and fixtures |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked -p cyberchef-rust-tests --doc` | PASS, 2 executing macro documentation examples, macOS ARM64 |
| Native Windows default/all-feature tests and installers | NOT TESTED |
| New complete GitHub Actions run on the modified commit | NOT TESTED |
| External publication/install verification | NOT TESTED; not authorized |

No native Windows binaries or CI release assets were generated by this audit;
there are no Windows artifact sizes/hashes or installation results to report.

## Later packaging configuration enhancement

During this stabilization, DEB/RPM/AUR workflows were further configured to
exercise installed TUI PTY behavior and uninstall/reinstall functional checks,
with exact RPM/AUR CLI package globs. Workflow syntax/regression checks passed;
these native package-manager scenarios remain NOT TESTED locally.

## Mandatory native all-feature gate preparation

The original native workflow tested only default workspace features. The
updated `scripts/check-native-platform.sh` now unconditionally runs:

```sh
cargo test --locked --workspace
cargo check --locked --workspace --all-targets --all-features
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D clippy::correctness -D clippy::suspicious
```

No ignore flags, optional-job conditions or failure masking were added. Native
matrix timeout is 180 minutes to accommodate the additional native libraries
and complete optional-feature compilation.

Prerequisite setup in `platform-checks.yml`:

- Linux installs Tesseract/Leptonica development packages, English OCR data,
  clang/cmake/libclang alongside existing desktop dependencies.
- macOS installs Homebrew tesseract/pkgconf; Xcode provides native compilers.
- Windows uses the runner's vcpkg executable, records the exact vcpkg Git
  revision and package inventory, and installs `tesseract:x64-windows-static`
  plus `pkgconf:x64-windows-static` into an isolated temporary prefix.
  `PKG_CONFIG` names the native pkgconf.exe; `PKG_CONFIG_PATH` and
  `PKG_CONFIG_LIBDIR` select only the static prefix, and
  `PKG_CONFIG_ALL_STATIC=1` requests the full static link closure. The workflow
  probes tesseract.pc and static linker flags before Cargo runs. The vcpkg
  static triplet selects static libraries and CRT, matching Rust +crt-static.
  The English language model is fetched from immutable tessdata_fast commit
  `65727574dfcd264acbb0c3e07860e4e9e9b22185` (4.1.0), its actual SHA-256 is logged,
  and `TESSDATA_PREFIX` supplies it to the mandatory OCR runtime test.

This follows locked tesseract-rs 0.4.0's `use-system-tesseract` build contract:
its build script calls pkg-config for tesseract, with no vcpkg auto-discovery.
It does not suffice to install a Windows OCR application with only DLLs.
Upstream vcpkg confirms the Tesseract port installs/fixes pkg-config metadata
and copies its tool, pkgconf installs under tools/pkgconf, and the static
triplet selects a static CRT:
[Tesseract port](https://raw.githubusercontent.com/microsoft/vcpkg/master/ports/tesseract/portfile.cmake),
[pkgconf port](https://raw.githubusercontent.com/microsoft/vcpkg/master/ports/pkgconf/portfile.cmake),
[static triplet](https://raw.githubusercontent.com/microsoft/vcpkg/master/triplets/x64-windows-static.cmake).
Model source:
[tessdata_fast 4.1.0](https://github.com/tesseract-ocr/tessdata_fast/releases/tag/4.1.0).

PASS: actionlint, Bash syntax validation of the native script and git diff
whitespace check. No Cargo/npm commands or native prerequisite installations
were run during this preparation. Windows package installation, pkg-config
closure/MSVC linking, actual English OCR results, native all-feature tests and
the overall same-commit native matrix remain **NOT TESTED**. The runner-supplied
vcpkg baseline is recorded but not yet pinned in repository configuration;
port availability and build time must be proven on the target before claiming
Windows all-feature support or reproducibility. The mandatory gates fail if
that provisioning is incomplete.
