# Packaging and crates.io publication rehearsal

Date: 2026-10-17 (Europe/Berlin). Base commit: `d07213d86e0849239cb169489221fea5ff7d8411`.
Version: `0.2.0-rc.1`. Evidence applies to the current uncommitted working tree,
including the fixes described below, rather than that immutable commit alone.
Host: Darwin arm64 (macOS-26.4-arm64-arm-64bit).
Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14) (Homebrew)`; `cargo 1.97.1 (c980f4866 2026-06-30) (Homebrew)`. Tar: `bsdtar 3.5.3 - libarchive 3.7.4 zlib/1.2.12 liblzma/5.4.3 bz2lib/1.0.8`.

## Changes and regression results

- PASS: `python3 scripts/test-distributions.py` — 11 isolated regressions using real host BSD tar. Tests cover explicit members/checksums for CLI, TUI, combined, headless and web variants; no-overwrite checks; existing checksum/dangling symlink; failed build/tar cleanup; archive-publication race rollback; simulated Windows names; Cargo-derived smoke version; Homebrew CLI/TUI routing against the installed keg; uninstall after failed smoke.
- PASS: `python3 scripts/test-publish-crate.py` — 10 isolated regressions. Cargo publication and HTTP requests are mocked, so these tests perform no publication. Tests cover exact sparse-index propagation and retries, already-published version verification, mandatory dry-run execution, invalid package/argument rejection, network/429/5xx/malformed response failure, invalid/yanked index failure, bounded missing-index timeout, token requirement and Cargo failure.
- PASS: Bash syntax checks for build-distributions.sh, publish-crate.sh, test-homebrew-formula.sh, release-gates.sh and check-native-platform.sh.
- Both publication regression and distribution regression run in the release and native-platform gates.

The tar archive is now written beside its content directory and archives an explicit list of binaries and LICENSE. `COPYFILE_DISABLE=1` prevents BSD tar from injecting macOS AppleDouble sidecars. Complete checksum and archive files are published through non-overwriting hardlinks on the destination filesystem, with the archive acting as the final commit marker. A failed archive publication removes only its newly created checksum; temporary directories are removed on normal failure and handled INT/TERM. As with ordinary filesystem transactions, SIGKILL/power-loss recovery is outside this script's guarantees.

Homebrew smoke resolves the actual installed keg binary. CLI runs a Base64 transformation; standalone TUI runs the existing graph PTY acceptance flow instead of unsupported CLI commands. This routing has fixture regression coverage; it is not evidence of an actual released-formula installation.

## Package contents

Command for each row: `cargo package --locked --allow-dirty --list -p <package>`.
These commands list package payloads without compiling or uploading.

| Package | Result | Files | Inspection |
| --- | --- | ---: | --- |
| `rxchef` | PASS | 508 | LICENSE, README, manifests, lock and sources present |
| `rxchef-store` | PASS | 17 | LICENSE, README, manifests, lock and sources present |
| `rxchef-client` | PASS | 17 | LICENSE, README, manifests, lock and sources present |
| `rxchef-cli` | PASS | 34 | LICENSE, README, manifests, lock and sources present |
| `rxchef-tui` | PASS | 11 | LICENSE, README, manifests, lock and sources present |

All five manifests declare Apache-2.0, repository/homepage, descriptions and README files. Each dependent crate now includes a plain `LICENSE` file whose bytes match the root Apache license, while retaining inherited SPDX `license = "Apache-2.0"` metadata. This avoids Cargo's warning about simultaneous `license` and `license-file` fields and avoids Windows symlink requirements. The final package-list checks contain LICENSE and emitted no license warnings. Core's explicit allowlist excludes repository tooling, verification artifacts and unrelated application sources. Internal runtime dependencies have version and path specifications aligned to `0.2.0-rc.1`.

## Publication controls and remaining gates

- PASS (static audit): dependency order is rxchef → rxchef-store → rxchef-client → rxchef-cli → rxchef-tui. Each required workspace dependency precedes its consumer.
- PASS (static audit): reusable crates.io workflow uses the release environment, read-only repository permission and explicit OIDC token permission. Stable-tag/prerelease guards remain. CI owner handles caller gates and workflow changes separately.
- PASS (regression): only HTTP 404 proves a version absent. Registry/network failures abort before publication. Already-existing versions also require an exact, non-yanked entry from `https://index.crates.io/rx/ch/<crate>` before returning; an API response alone is insufficient.
- NOT TESTED: actual crates.io publication, OIDC exchange and bootstrap token permissions. No registry writes were performed.
- NOT TESTED: complete registry-backed dry-runs for the four dependent crates. Their same-version internal dependencies must first exist in crates.io; package lists do not establish publish verification. First publication still requires a narrowly scoped publish-new bootstrap token and explicit maintainer authorization. Configure Trusted Publishing for each created crate before removing the bootstrap secret.
- PASS: native BSD tar and locally built GNU tar 1.35 both pass all 11 packaging regressions on this macOS ARM64 host. NOT TESTED: native Windows/Git Bash execution. Windows filename coverage above is simulated, not native platform acceptance.
- NOT TESTED in this subtask: actual variant builds, archive installed-binary execution, public Homebrew installation, DEB/RPM/AUR/container/native installer matrix. The parent release gate and platform jobs must supply separate evidence and artifact SHA-256 inventories.

Decision for this subtask: NOT READY FOR RELEASE; these packaging regressions and package lists alone cannot satisfy the complete platform and installer acceptance matrix.

## GNU tar follow-up (same host)

GNU tar was absent from PATH. Pinned GNU tar 1.35 was fetched from the official
GNU HTTPS source, [tar-1.35.tar.xz](https://ftp.gnu.org/gnu/tar/tar-1.35.tar.xz),
into `target/tools/gnu-tar-1.35/`; no global installation was performed.
Source archive SHA-256:
`4d62ff37342ec7aed748535323930c7cf94acf71c3591882b26a7ea50f3edc16`.
This records the downloaded bytes; detached-signature verification was not performed.

The local tool build used:

```sh
PATH=/usr/bin:/bin:/usr/sbin:/sbin ./tar-1.35/configure \
  --prefix="$PWD/installed" --disable-nls --disable-dependency-tracking LIBS=-liconv
make -j2
```

Both final commands passed. The initial configure attempt needed dependency
tracking disabled; the initial link needed macOS `-liconv`. These adjustments
apply only to the isolated tool build, without RX-Chef source changes.
The executable identifies itself as `tar (GNU tar) 1.35`.
Executable SHA-256: `edc593c54ff5332558bd9e59a1130ef0cc51866e8dd072e1f041c39398ad1e71`.

From the repository root:

```sh
PATH="$PWD/target/tools/gnu-tar-1.35/src:$PATH" python3 scripts/test-distributions.py
```

PASS: 11 tests in 6.805 seconds. The fixture prepends only mocked build-command
helpers; archive tests otherwise inherit this PATH and execute the real GNU tar
binary. The intentional tar-failure test replaces tar with its failure fixture.
Log: `target/tools/gnu-tar-1.35/regression.log`. No real Cargo/npm builds,
Homebrew installation, registry writes, or publication were invoked.
This establishes GNU tar command compatibility on macOS ARM64; it does not certify
Linux native installers or Windows/Git Bash.
