# Releasing

## Deliverables and first-release acceptance

| Deliverable | Local build/start | Release output |
|---|---|---|
| CLI | `cargo build --locked --release -p rxchef-cli`; `target/release/rxchef --help` | CLI archive/installers and packaged command help |
| TUI | `cargo build --locked --release -p rxchef-tui`; `target/release/rxchef_tui` | Separate TUI archive/native packages |
| Native HTTP server | `cargo build --locked --release -p rxchef-server` | Headless server archive |
| Browser workbench server | Build web assets, then server with `--features web-ui` | Separate embedded-web archive |
| Desktop GUI | `npm --prefix apps/web run desktop:build` | Tauri native app/installers |
| Container | `docker build -f docker/Dockerfile.server --target web -t rxchef-server:local .` | Locally built web/headless images; container CI on AMD64/ARM64 |

The [Docker guide](../server/docker.md) covers host-network local use and real
TLS-proxy deployment. Container CI currently builds/tests without an image-push step;
no public registry image is advertised before publication. CLI/TUI/server help,
all five shell completions and the generated manual are checked by
`scripts/test-command-help.py` in both native CI and the host release gate.
[Local storage](../workbench/local-storage.md) documents what survives restart
and what must be saved explicitly by each client.

For the first public release, record native install/launch/uninstall/reinstall,
GUI dialogs/offline work, byte-exact saves and missing platform evidence before
tagging. A green host gate and a configured image workflow are individual
checks, not automatic approval of all native products.

## Current release rehearsal

The current rehearsal version is `0.2.0-rc.1`; the stable successor is `0.2.0`
only after all acceptance gates pass. Remote tags `v0.0.1` and `v0.1.0` already
exist and must never be moved. Read-only checks on 2026-10-17 found no GitHub
Release and none of the five crates.io packages. Recheck version availability
immediately before publication. The tap could not be inspected (adapter 404).

The [current readiness report](first-public-release-readiness.md) supersedes
prior reports. Prior UI/publication authorization recorded in historical documents
does not authorize publishing this candidate. All external writes need explicit
maintainer approval. `.release/ui-acceptance.json` is pending for this version.

`scripts/check-release-tag.sh v0.2.0-rc.1 HEAD` validates a tag string without
creating a Git tag; publication validation must reject pending acceptance.
Run `python3 scripts/test-release-version.py` for the SemVer and clean-fixture gates.

The actual cargo-dist 0.32.0 plan includes CLI, TUI and the default headless
server. Desktop is explicitly excluded from cargo-dist and uses Tauri bundles.
The `build-workbench` global artifact job separately builds headless/web snapshots
and native GUI packages on both macOS architectures, both Linux architectures,
and Windows x86_64. These configured jobs require native CI execution; YAML alone
is not platform certification. Runner labels follow the
[GitHub runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

For a local rehearsal install pinned `cargo-audit` 0.22.2 and `dist` 0.32.0, then run
`bash scripts/release-gates.sh host`. Dependency audits are mandatory: RustSec or npm audit failures block release.
The Linux gate image installs the pinned audit tool; host users need `cargo audit`
on PATH. The gate builds from current web source, checks
both Rust feature sets, runs real browser/PTY tests, creates fresh local archive
variants, tests temporary install/uninstall/reinstall, builds host GUI bundles
and checks dist plan/build. `SKIPPED`, `NOT TESTED` and `FAIL` remain separate;
host PASS covers only executed host gates. GUI interaction, network-disabled
offline acceptance and other native platforms still require their own evidence.
The GitHub rehearsal includes the web/desktop package matrix and defaults to no
artifact uploads. Retention in GitHub Actions storage requires explicitly opting
in with `retain-artifacts`; no such workflow was dispatched during local testing.
All commands can be wrapped in `scripts/record-release-check.py --label NAME -- COMMAND`
to retain JSON metadata and complete logs under `target/rc-evidence`.

`scripts/build-distributions.sh` refuses existing archive names, including stale
ones. Use a fresh `RXCHEF_DISTRIBUTION_DIR` for another rehearsal. Run
`python3 scripts/inventory-release-artifacts.py` to record SHA-256 and sizes;
inventory creation alone does not claim installation or smoke success. The
temporary archive smoke results are recorded separately by
`python3 scripts/smoke-distributions.py`. Local macOS bundles are unsigned and
not notarized for public distribution; Windows/Linux installer and signing
claims require tests on those native systems.

Protocol version 2 is intentional across JSONL and HTTP clients/servers. Update
both sides together; incompatible remote capabilities must fail explicitly.

Pushing a signed, correctly-versioned stable tag (`vX.Y.Z`) triggers the
release: platform binaries, a GitHub Release, a shell installer, `.deb` and
`.rpm` packages, a Homebrew tap update, and five crates published to
crates.io. This page is the maintainer's reference for how that pipeline
works, what has to be set up once before the first real release, and the
actual commands to run.

**Scope decision:** only Homebrew and crates.io are published to an
external package index. AUR is built and functionally tested on every
release (`packaging/aur/PKGBUILD` inside a real Arch container) and the
rendered `PKGBUILD`/`.SRCINFO`/built packages are attached to the GitHub
Release as artifacts, but nothing is pushed to the real
`aur.archlinux.org` remote — that's a deliberate "local" release: versioned
in this repo, not published to an external index, until an AUR account and
registered SSH key exist.

## The pipeline

```text
git push origin vX.Y.Z
        │
        ▼
.github/workflows/release.yml (generated by `dist`)
        │
        ├── validate-tag        scripts/check-release-tag.sh on the tag SHA
        ├── release-gates       .github/workflows/platform-checks.yml, re-run
        │                       on the tag SHA — never trust that master
        │                       was green when the tag was cut
        │
        ├── build-local-artifacts     per-platform archives (5 targets)
        ├── build-global-artifacts    checksums, shell installer, source tarball
        ├── build-deb                 .deb, built + installed + smoke-tested
        ├── build-rpm                 .rpm, built + installed + smoke-tested
        ├── build-aur                 PKGBUILD rendered, built, smoke-tested —
        │                             attached to the release, not pushed to AUR
        │
        ├── host                 creates the GitHub Release, uploads everything
        │
        ├── verify downloaded public assets (exact file set, sizes, SHA-256)
        ├── publish-crates             rxchef → rxchef-store → rxchef-client → rxchef-cli → rxchef-tui
        │     └── install exact published CLI/TUI versions and functionally test
        ├── publish-homebrew-formula   only after registry verification succeeds
        │
        └── announce
```

Platform checks run on pull requests and pushes to `master`. The complete
release graph runs on a version tag, with release-specific validation skipped
on pull-request rehearsals. Publishing steps only fire on a real tag. The publish jobs run only on tags, and only for **stable** tags
(`v0.2.0-rc.1`-style prerelease tags get a GitHub prerelease, archives, and
installers, but never touch the Homebrew tap or crates.io — see
`publish-prereleases` in `dist-workspace.toml`, which defaults to `false`).

`dist-workspace.toml` configures packaging and custom jobs.
The repository maintains `.github/workflows/release.yml` from the cargo-dist
0.32.0 baseline with a stricter publication condition. `allow-dirty = ["ci"]`
prevents regeneration from restoring the upstream condition that accepts skipped
builds after a failed tag validator. Every required validation/build must succeed;
`test-release-version.py` checks these dependencies and rejects skipped, cancelled
or failed gates. Update the workflow deliberately when changing dist configuration. The
custom jobs (`validate-tag`, `release-gates`, `build-deb`, `build-rpm`,
`build-aur`, `publish-crates`) are `dist`'s documented extension points
(`plan-jobs`, `local-artifacts-jobs`, `global-artifacts-jobs`,
`publish-jobs`) and live in their own workflow files under
`.github/workflows/`, which `dist generate` does not touch.

## Package identity

| Published as | Crate identifier in code | Binary |
|---|---|---|
| `rxchef` (library) | `rxchef` | — |
| `rxchef-store` | `rxchef_store` | — |
| `rxchef-client` | `rxchef_client` | — |
| `rxchef-cli` | `rxchef_cli` | `rxchef` |
| `rxchef-tui` | `rxchef_tui` | `rxchef_tui` |

The crates.io package names use hyphens; the `[package.metadata.dist]`
`formula = "rxchef"` override on `rxchef-cli` is what makes the Homebrew
formula and AUR package match the binary name (`rxchef`) rather than the
package name (`rxchef-cli`).

## One-time setup (not automatable — do these once, in order)

### 1. GitHub environment and tag protection

Create a GitHub Environment named `release` (repo Settings → Environments).
Every secret below belongs there, not as a bare repository secret, so a
workflow can only reach them from a job that explicitly declares
`environment: release`.

Add a Ruleset (repo Settings → Rules → Rulesets) restricting who can push to
`refs/tags/v*` and to `master`, so a release tag can only be created by a
maintainer. Without this, anyone with write access can trigger a full
publish across five external systems by pushing a tag.

Custom publish jobs explicitly grant `contents: read` and `id-token: write`
through cargo-dist's
[`github-custom-job-permissions`](https://axodotdev.github.io/cargo-dist/book/reference/config.html#github-custom-job-permissions)
configuration. An explicit job permission map replaces defaults; the caller must
cover the reusable publisher's requested scopes. Release-version regression tests
compare these scopes so GitHub does not reject the workflow before jobs start.

### 2. crates.io: bootstrap, then Trusted Publishing

crates.io Trusted Publishing (GitHub OIDC, no stored token) can only be
configured for a crate that already exists on the registry — so the very
first publish of each of the five crates needs a real, scoped token:

1. Create a crates.io token scoped to publish-new, add it to the `release`
   environment as `CRATES_IO_BOOTSTRAP_TOKEN`.
2. Push the first stable tag. `publish-crates.yml` detects the secret and
   uses it directly for all five crates, in order.
3. On crates.io, for each of the five crates, add a Trusted Publisher:
   repository `MichaelWeissDEV/rx-chef`, workflow `release.yml`, environment
   `release`. The current registry validates `workflow_ref` (the caller filename),
   as shown in its [GitHub claims implementation](https://github.com/rust-lang/crates.io/blob/33e1ca9abb94e17779d580bd37516c6db1bcd1ab/crates/crates_io_trustpub/src/github/claims.rs).
   Recheck the live configuration after bootstrap; YAML cannot prove the account settings.
4. Delete `CRATES_IO_BOOTSTRAP_TOKEN` from the `release` environment. Every
   later publish now goes through OIDC via `rust-lang/crates-io-auth-action`
   automatically — `publish-crates.yml` checks for the secret and falls back
   to Trusted Publishing when it's absent.

`scripts/publish-crate.sh` makes this idempotent: it skips a `package@version`
that's already live, so a re-run after a partial failure doesn't error out
re-publishing what already succeeded, and waits for registry propagation
before the next, dependent crate publishes.

### 3. Homebrew tap

Create `MichaelWeissDEV/homebrew-tap` (empty is fine; `dist` publishes into
it). Create a token scoped to just that one repository's contents, add it
to the `release` environment as `HOMEBREW_TAP_TOKEN`.

### AUR (optional, not required for a release)

AUR publishing is out of scope for now (see the scope decision at the top
of this page): `build-aur` builds and functionally tests the package on
every release without needing any AUR account or secret. If real AUR
publishing is wanted later, that means: creating an AUR account, adding a
dedicated CI SSH key to it, adding `AUR_SSH_PRIVATE_KEY` and
`AUR_KNOWN_HOSTS` (verified against a published AUR SSH host key
fingerprint, not blindly trusted from `ssh-keyscan`) to the `release`
environment, restoring `"./publish-aur"` to `publish-jobs` in
`dist-workspace.toml`, and reinstating the push step this repository's
history shows was removed from what is now `build-aur.yml`.

## Release checklist

```text
[ ] version bumped in Cargo.toml (workspace.package.version)
[ ] CHANGELOG.md: [Unreleased] renamed to [X.Y.Z] - YYYY-MM-DD
[ ] master is green (cargo test --workspace, cargo test --workspace --all-features)
[ ] scripts/release-gates.sh host passes
[ ] cargo run -p xtask -- audit-operations reports 478/478 (or current total) verified
[ ] cargo run -p xtask -- docs --check and generate_operation_docs --check pass
[ ] Cargo.lock committed and current (cargo metadata --locked succeeds)
[ ] git tree is clean (git status --porcelain is empty)
[ ] release dry run (below) is green
[ ] cargo package --list / cargo publish --dry-run reviewed for each crate
[ ] git config user.signingkey is set (required for `git tag -s`)
```

## Rehearsing before you tag: the dry run

`.github/workflows/release-dry-run.yml` is a `workflow_dispatch`-only
workflow that runs the entire pipeline — gates, `dist plan`/`dist build`,
`.deb` and `.rpm` builds, an AUR `makepkg` build against the current commit,
Homebrew formula generation, and `cargo publish --dry-run` for
`rxchef` — with **no GitHub Release and no crates.io or
Homebrew upload** (AUR is never pushed externally regardless — see the
scope decision above). Trigger it from the Actions tab before cutting a
real tag.

(`rxchef-store`/`rxchef-client`/`rxchef-cli`/`rxchef-tui` can't be dry-run publish-checked this way:
`cargo publish` resolves path dependencies against the real registry even
in `--dry-run` mode, so until their dependencies (including `rxchef-client` for the TUI) are actually live,
those packages error with "no matching package found" — this is a real Cargo
limitation, not a config problem. `cargo package --list` still validates
their packaged contents.)

## The actual release

```bash
git checkout master
git pull --ff-only

# CHANGELOG.md: rename [Unreleased] to [X.Y.Z] - YYYY-MM-DD, commit it.
# Bump Cargo.toml's workspace.package.version to match, commit it.

./scripts/release-gates.sh host

git tag -s vX.Y.Z -m "rx-chef vX.Y.Z"
git push origin vX.Y.Z
```

CI takes it from there. Inspect the final job results and then download and
verify the public files and perform real package-manager installations. A started
workflow or successful announcement is not proof that every public path works.

## Feature profile for distributed packages

Every distributed binary (archives, `.deb`, `.rpm`, the AUR package, the
Homebrew formula) is built with **default features only** — no OCR, PGP,
YARA, or disassembly backends, which pull in native system dependencies
(Tesseract, libyara, etc.) that would make `brew install`/`apt install`/
`pacman -S` unreliable across hosts. Operations gated behind those features
remain discoverable in `rxchef operations` and report why they're
unavailable; users who need them build from source with `--features full`
(see `docs/reference/feature-matrix.md`).

## UI acceptance before publication

The previous `0.0.1` acceptance is historical. `.release/ui-acceptance.json`
now records pending review for `0.2.0-rc.1`. Automated browser success alone
does not constitute maintainer acceptance.

1. Review the built production desktop app and the matching server-served UI,
   including compact menus, panel docking, Auto-Bake, multiple outputs,
   transformation reports and selective project export/import.
2. After explicit acceptance, record `status: "accepted"`, the complete reviewed
   commit SHA in `accepted_commit`, the reviewer and an ISO-8601 `reviewed_at`
   time with timezone. Keep the version equal to Cargo's complete version.
3. Commit the acceptance record. `python3 scripts/check-ui-acceptance.py` checks
   that the reviewed commit is an ancestor and its GUI source matches the
   current release source. Later GUI/dependency/native changes require review
   again. Documentation-only acceptance commits can retain the reviewed SHA.
4. Run **Release dry run** with artifact retention if reviewable CI packages are
   wanted. Pending UI acceptance permits rehearsal, while the summary now waits
   for desktop/workbench packaging and fails for cancelled/skipped required jobs.
5. Only after the UI and the remaining platform/installation release criteria
   pass, create/push the version tag according to the workflow above. The tag
   validation job enforces UI acceptance before builds/publication proceed.
   Planning runs `dist plan` with read-only permissions. In cargo-dist 0.32.0,
   GitHub `host --steps=create` computes hosting metadata rather than creating a
   remote draft; the actual release write runs in `host` after all required
   gates succeed, with `contents: write` and the `release` environment.

The maintained cargo-dist release workflow builds archives/installers and
attaches workbench packages, with prereleases excluded from stable package
publication. Validate release rules with `python3 scripts/test-release-version.py`,
`python3 scripts/test-ui-acceptance.py` and `actionlint`. `dist generate --check`
checks the other generated surfaces; CI generation is deliberately disabled.

Tag validation installs `rustfmt` explicitly: the minimal Rust profile does not
include the formatter used by `xtask check-registry`. Registry diagnostics remain
visible so a missing tool is not misreported as stale generated code.

`precise-builds = true` builds distributable CLI/TUI/server crates separately.
The default workspace-wide binary build also includes the Tauri desktop, which
requires GTK/WebKit on Linux and belongs to `build-workbench.yml` instead.
