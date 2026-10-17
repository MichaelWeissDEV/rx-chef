# Dependency audit and minimal remediation

Decision: **NOT READY FOR RELEASE**. The updated workspace audit still reports
11 vulnerability entries. These are advisory/version matches, not 11 confirmed
exploits in RX-Chef. No exploit validation was performed.

Date: 2026-10-17 (Europe/Berlin). Base commit
`d07213d86e0849239cb169489221fea5ff7d8411`; working version `0.2.0-rc.1`.
All results apply to the modified working tree, before a final release commit.
No package version fields, remote repositories or registries were changed by
this dependency remediation.

## Evidence and scope

Initial input:
`target/first-public-evidence/20261017T172641575533Z-rustsec-audit.log`.
It records 27 vulnerability entries in 1045 locked dependencies against 1338
RustSec advisories, DB commit `999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b`
(last update `2026-10-17T17:23:59+02:00`). Duplicate advisory IDs against different
locked package versions remain separate entries in the inventory below.

Analysis used `cargo tree --locked --offline --workspace --all-features --invert
<package> --depth 4`, cached crates.io sparse-index metadata, locked upstream
sources and product callers. The cached index confirms the proposed versions
exist, are not yanked, and satisfy the listed upstream semver constraints.
Registry metadata is local evidence at this audit time, not a guarantee about
future upstream releases. There is no root SECURITY.md; reachability notes use
actual package/features and source call sites, rather than assuming a formal
security policy or treating a lockfile match as exploit proof.

Post-change commands (exit nonzero because advisories remain):

```sh
target/release-tools/bin/cargo-audit audit --json --db target/rustsec-advisory-db --no-fetch
target/release-tools/bin/cargo-audit audit --json --db target/rustsec-advisory-db --no-fetch --file fuzz/Cargo.lock
```

Saved outputs:
`target/first-public-evidence/dependency-remediation-audit.json` (11 vulnerability
entries) and `dependency-remediation-fuzz-audit.json` (1 vulnerability entry).
The local DB still contains 1338 advisories, but no-fetch output omits its Git
revision/timestamp. No ignore list or severity filtering was introduced.
A fresh network-enabled dependency/yank audit remains required before release.

## Applied minimal updates

| Dependency | Before → after | Why the update is realistic |
| --- | --- | --- |
| crossbeam-epoch | 0.9.18 → 0.9.20 | Patched minimum; crossbeam-deque 0.8.6 accepts ^0.9.17. MSRV 1.61. |
| h2 | 0.4.15 → 0.4.16 | Patched minimum; hyper 1.10.1 accepts ^0.4.14. MSRV 1.63. |
| quinn-proto | workspace 0.11.14 / fuzz 0.11.16 → 0.11.18 | Covers all five supplied advisories; compatible 0.11 line, MSRV 1.85. |
| rustls | workspace 0.23.40 / fuzz 0.23.43 → 0.23.46 | Covers all four supplied advisories; reqwest accepts ^0.23.4, MSRV 1.71. Required aws-lc and webpki transitive updates accepted. |
| quick-xml | 0.39.4 and 0.40.1 → 0.41.0 | Direct lower bound raised to 0.41.0. plist 1.9.0 constrained the old 0.39 line; raising plist lower bound to 1.10.0 and locking 1.10.0 allows one patched 0.41 version. MSRV 1.79 / plist 1.88. |
| lz4_flex test dependency | 0.9.5 → 0.13.1 | Runtime already used patched 0.13.1; tests now share it. The old vulnerable 0.9 version was test-only. |
| direct zip dependency | 8.6.0 → 9.0.4 | Root/test minimum raised to 9.0.0. ZIP9 requires MSRV 1.88 and updates compression dependencies. Old upstream ZIP8 remains separately, not hidden. |
| anyhow | 1.0.102 → 1.0.103 | Fixes the supplied downcast_mut soundness warning; compatible patch. Fuzz already had 1.0.104, so it was normalized to the same patched version. |
| memmap2 | 0.9.10 → 0.9.11 | Fixes range/pointer soundness warning; compatible patch. Absent from fuzz lock. |

Both workspace and fuzz locks were updated. `plist = "1.10.0"`,
`quick-xml = "0.41.0"`, `zip = "9.0.0"`, and test `lz4_flex = "0.13.1"`
provide safe minimums for those direct dependencies when downstream packages
resolve their own locks. Transitive patched selections are captured by the
locks; no unnecessary direct dependencies were added to constrain upstream
internals.

ZIP9 compatibility edits are limited to `src/operations/zip.rs`,
`src/operations/unzip.rs`, and `tests/tests/operations/unzip.rs`:
FileOptions now has two lifetimes, and `ZipFile::name()` returns a fallible Cow.
Filename decoding errors are propagated as operation errors. quick-xml/plist
required no source API changes in the targeted check.

## Reachability assessment and remaining blockers

- **crossbeam-epoch:** runtime image processing uses Rayon → rayon-core →
  crossbeam-deque; YARA also has a build-time ignore/globwalk path. The advisory
  needs specific pointer formatting. No exact formatting trigger was established.
  Version was updated without claiming a reachable pointer-dereference exploit.
- **h2 / rustls:** reqwest is used by core HTTP operations and remote clients,
  so attacker-influenced remote traffic can reach these networking libraries.
  Server axum also resolves hyper/h2. Individual protocol/feature preconditions
  (HTTP/2 negotiation, certificate compression, key exchange and TLS Debug
  logging) were not validated. Patched versions are preferable to relying on
  those conditions being absent.
- **quinn-proto:** present in locks via optional reqwest HTTP/3 dependencies, but
  `cargo tree --workspace --all-features --invert quinn-proto` prints no active
  path in this configuration. Workspace --all-features does not automatically
  enable every feature of registry dependencies. Patched anyway; no reachable
  QUIC runtime service was established.
- **quick-xml:** direct XML Minify/Beautify parse input using Reader, not NsReader.
  plist 1.9 also used Reader. The namespace-specific advisory therefore has no
  direct NsReader call established here; duplicate-attribute checking and
  upstream consumers still require detailed tracing. Both versions were removed.
- **lz4_flex 0.9.5:** only cyberchef-rust-tests selected it. Runtime decompression
  already used 0.13.1. Removing the old test dependency removes that lock match.
- **rsa 0.9.10:** no patched version is provided by RUSTSEC-2023-0071. Core RSA
  Decrypt and RSA Sign load caller-supplied private keys and perform private-key
  operations (`src/operations/rsa_decrypt.rs`, `rsa_sign.rs`). Repeated timing
  observations involving a reused secret key need assessment; there is no
  evidence here of a server-managed secret key service, nor proof that timing
  side channels are impossible for downstream library users. This remains an
  explicit unresolved risk, not a suppressed advisory. Replacing the backend
  or restricting private-key use would be a separate design change.
- **Wasmtime 43.0.2:** seven advisory matches remain via optional YARA-X 1.18.0.
  Default release packages omit YARA; all-feature/source builds include it.
  YARA requests only Wasmtime cranelift/runtime with default features disabled,
  compiles its own core Wasm and uses one global Engine. No WASI sandbox or
  component-model/async API use was established in the inspected path, so the
  component/WASI advisories are not automatically reachable. Recursive types
  and engine/type-index invariants still need specific analysis. These remain
  needs-review upstream matches, not confirmed sandbox escapes in RX-Chef.
  The current 43.x requirement cannot resolve patched 48.0.6 or 49.0.3. Cached
  metadata for latest YARA-X 1.21.0 still requests Wasmtime ^45.0.3, which also
  fails the supplied patched ranges. A blind YARA update is not a remedy;
  upstream-supported migration or reviewed backports are required.
- **ZIP8.6.0:** the three filesystem extraction advisories remain via YARA-X
  (^8.2.0) and Tesseract's build dependency. The application Unzip operation
  reads entries into byte buffers and does not call extract or write archive
  entries to disk. YARA's CRX parser likewise reads ZipArchive entries.
  Tesseract build extraction loops entries with mangled_name and explicit file
  writes, rather than the affected generic extract APIs. These are concrete
  counterindications for the supplied exact API claims, but not a complete
  review of every upstream ZIP use. Latest cached YARA-X 1.21.0 still requests
  ZIP8; do not force a semver-incompatible ZIP9 override into upstream crates.
  Upstream migration/backport and independent call-site review remain open.

## Original 27-entry inventory

“Updated” means the affected package/version match is absent from the new audit;
it does not constitute runtime exploit validation or certify all future versions.
“Remains” requires the review described above.

| Advisory | Original package/version | Patch range from supplied advisory | Outcome |
| --- | --- | --- | --- |
| [RUSTSEC-2026-0204](https://rustsec.org/advisories/RUSTSEC-2026-0204.html) | crossbeam-epoch 0.9.18 | >=0.9.20 | Updated |
| [RUSTSEC-2026-0258](https://rustsec.org/advisories/RUSTSEC-2026-0258.html) | h2 0.4.15 | >=0.4.16 | Updated |
| [RUSTSEC-2026-0041](https://rustsec.org/advisories/RUSTSEC-2026-0041.html) | lz4_flex 0.9.5 | >=0.11.6, <0.12.0; >=0.12.1 | Updated |
| [RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html) | quick-xml 0.39.4 | >=0.41.0 | Updated |
| [RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html) | quick-xml 0.39.4 | >=0.41.0 | Updated |
| [RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html) | quick-xml 0.40.1 | >=0.41.0 | Updated |
| [RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html) | quick-xml 0.40.1 | >=0.41.0 | Updated |
| [RUSTSEC-2026-0342](https://rustsec.org/advisories/RUSTSEC-2026-0342.html) | quinn-proto 0.11.14 | >=0.11.17 | Updated |
| [RUSTSEC-2026-0339](https://rustsec.org/advisories/RUSTSEC-2026-0339.html) | quinn-proto 0.11.14 | >=0.11.17 | Updated |
| [RUSTSEC-2026-0338](https://rustsec.org/advisories/RUSTSEC-2026-0338.html) | quinn-proto 0.11.14 | >=0.11.18 | Updated |
| [RUSTSEC-2026-0340](https://rustsec.org/advisories/RUSTSEC-2026-0340.html) | quinn-proto 0.11.14 | >=0.11.18 | Updated |
| [RUSTSEC-2026-0185](https://rustsec.org/advisories/RUSTSEC-2026-0185.html) | quinn-proto 0.11.14 | >=0.11.15 | Updated |
| [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071.html) | rsa 0.9.10 | No patched version | Remains |
| [RUSTSEC-2026-0346](https://rustsec.org/advisories/RUSTSEC-2026-0346.html) | rustls 0.23.40 | >=0.23.46 | Updated |
| [RUSTSEC-2026-0285](https://rustsec.org/advisories/RUSTSEC-2026-0285.html) | rustls 0.23.40 | >=0.23.45 | Updated |
| [RUSTSEC-2026-0348](https://rustsec.org/advisories/RUSTSEC-2026-0348.html) | rustls 0.23.40 | >=0.23.46 | Updated |
| [RUSTSEC-2026-0349](https://rustsec.org/advisories/RUSTSEC-2026-0349.html) | rustls 0.23.40 | >=0.23.46 | Updated |
| [RUSTSEC-2026-0362](https://rustsec.org/advisories/RUSTSEC-2026-0362.html) | wasmtime 43.0.2 | >=48.0.6, <49.0.0; >=49.0.3 | Remains |
| [RUSTSEC-2026-0222](https://rustsec.org/advisories/RUSTSEC-2026-0222.html) | wasmtime 43.0.2 | >=24.0.12, <25.0.0; >=36.0.13, <37.0.0; >=46.0.2, <47.0.0; >=47.0.3 | Remains |
| [RUSTSEC-2026-0327](https://rustsec.org/advisories/RUSTSEC-2026-0327.html) | wasmtime 43.0.2 | >=48.0.4, <49.0.0; >=49.0.2 | Remains |
| [RUSTSEC-2026-0316](https://rustsec.org/advisories/RUSTSEC-2026-0316.html) | wasmtime 43.0.2 | >=36.0.16, <37.0.0; >=48.0.3, <49.0.0; >=49.0.1 | Remains |
| [RUSTSEC-2026-0363](https://rustsec.org/advisories/RUSTSEC-2026-0363.html) | wasmtime 43.0.2 | >=36.0.18, <37.0.0; >=48.0.6, <49.0.0; >=49.0.3 | Remains |
| [RUSTSEC-2026-0269](https://rustsec.org/advisories/RUSTSEC-2026-0269.html) | wasmtime 43.0.2 | >=24.0.13, <25.0.0; >=36.0.14, <37.0.0; >=46.0.3, <47.0.0; >=47.0.4 | Remains |
| [RUSTSEC-2026-0359](https://rustsec.org/advisories/RUSTSEC-2026-0359.html) | wasmtime 43.0.2 | >=48.0.6, <49.0.0; >=49.0.3 | Remains |
| [RUSTSEC-2026-0367](https://rustsec.org/advisories/RUSTSEC-2026-0367.html) | zip 8.6.0 | >=9.0.0 | Remains |
| [RUSTSEC-2026-0368](https://rustsec.org/advisories/RUSTSEC-2026-0368.html) | zip 8.6.0 | >=9.0.0 | Remains |
| [RUSTSEC-2026-0369](https://rustsec.org/advisories/RUSTSEC-2026-0369.html) | zip 8.6.0 | >=9.0.0 | Remains |

## Other audit warnings

Post-update workspace output separately reports 8 unmaintained entries and
4 unsound entries: glib 0.18.5 (requires >=0.20), lru 0.12.5 (two advisories),
and lru 0.16.4 (requires >=0.18.2 for pop panic safety). These are not silently
counted as fixed by the 27→11 reduction. Unmaintained entries include bincode,
paste, proc-macro-error, rusttype, three ttf-parser versions and unsafe-libyaml.
GTK/Tauri and image/QR/terminal upstream constraints require separate upgrade
review; no broad framework changes were attempted. The initial full audit also
reported yanked chacha20 0.10.0 and der 0.8.0; those minimum dependencies require
an explicit fresh registry/yank check. No-fetch output must not be interpreted
as proving their yank status has changed.

## Validation and next steps

- PASS: `cargo check --locked -p rxchef -p cyberchef-rust-tests --all-targets`
  after the ZIP9 compatibility edits (macOS ARM64, Rust 1.97.1).
- PASS: `cargo fmt --all -- --check` and `git diff --check`.
- Audit remains FAIL: workspace 11 vulnerability matches, fuzz 1.
- NOT TESTED here: complete default/all-feature workspace runtime suites after
  dependency changes, native Windows/linking, Linux containers, installers,
  published-crate resolution and pinned CI Rust 1.96.1. Parent will run the
  full gate sequentially after this handoff.

Before release, run the full gates and fresh audits, resolve or explicitly
review the upstream/no-patch risks under maintainer policy, and record exact
artifact-feature exposure. No claim of READY FOR RELEASE follows from fewer
advisory matches or a successful compile check.
