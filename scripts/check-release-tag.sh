#!/usr/bin/env bash
#
# Validates that a release tag is safe to publish from.
#
# This is the single gate between "someone pushed a tag" and "the release
# workflow starts building artifacts". It exists so a typo'd tag, a stale
# Cargo.lock, or an out-of-sync CHANGELOG cannot silently trigger (or
# silently fail to trigger) a real release.
#
# Usage:
#   scripts/check-release-tag.sh <tag> [<commit-ish>]
#
#   <tag>         The tag name to validate, e.g. v0.1.0 or v0.2.0-rc.1.
#                 This does NOT need to exist as a real git tag object; the
#                 version/changelog/lockfile/docs/audit checks only need the
#                 string and the current worktree. This lets the whole gate
#                 be exercised before ever running `git tag`.
#   <commit-ish>  What to treat as "the tag commit" for the ancestry check.
#                 Defaults to HEAD. In CI this should be the actual tag SHA
#                 ($GITHUB_SHA); for local testing, HEAD (or any other
#                 existing commit) is a legitimate synthetic stand-in.
#
# Exit status is non-zero if any check fails; every check runs regardless of
# earlier failures, and the summary lists all of them.

set -uo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

tag=${1:-}
commit=${2:-HEAD}

if [[ -z "$tag" ]]; then
    echo "usage: $0 <tag> [<commit-ish>]" >&2
    exit 2
fi

failed=()
prerelease=false

fail() {
    printf '!!! %s\n' "$1" >&2
    failed+=("$1")
}

ok() {
    printf '==> %s\n' "$1"
}

# --- Tag shape ---------------------------------------------------------

# Stable: v0.1.0, v1.2.3. Prerelease: v0.2.0-rc.1, v0.2.0-beta.1.
tag_re='^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?$'
if [[ "$tag" =~ $tag_re ]]; then
    ok "tag shape ($tag)"
else
    fail "tag '$tag' does not look like v<major>.<minor>.<patch>[-<prerelease>]"
fi

if [[ "$tag" == *-* ]]; then
    prerelease=true
    ok "prerelease tag detected — will not gate on AUR/Homebrew-stable checks"
fi

tag_version=${tag#v}
version_core=${tag_version%%-*}

# --- Cargo.toml version match -------------------------------------------

workspace_version=$(
    cargo metadata --locked --format-version 1 --no-deps 2>/dev/null \
        | python3 -c "
import json, sys
data = json.load(sys.stdin)
# The root 'rxchef' package's version is the workspace version; every
# publishable crate inherits it via version.workspace = true.
versions = {p['name']: p['version'] for p in data['packages']}
print(versions.get('rxchef', ''))
"
)
if [[ -z "$workspace_version" ]]; then
    fail "could not read workspace.package.version from Cargo.toml"
elif [[ "$workspace_version" == "$version_core" ]]; then
    ok "tag version matches Cargo.toml workspace version ($workspace_version)"
else
    fail "tag version '$version_core' != Cargo.toml workspace version '$workspace_version'"
fi

# --- CHANGELOG has this exact version -----------------------------------

if grep -qE "^## \[$version_core\]" CHANGELOG.md; then
    ok "CHANGELOG.md has a [$version_core] section"
else
    fail "CHANGELOG.md has no '## [$version_core]' section (still under [Unreleased]?)"
fi

# --- Tag commit is reachable from master ---------------------------------

if ! git rev-parse --verify --quiet "$commit^{commit}" >/dev/null; then
    fail "commit-ish '$commit' does not resolve to a commit in this repository"
elif git merge-base --is-ancestor "$commit" origin/master 2>/dev/null \
    || git merge-base --is-ancestor "$commit" master 2>/dev/null; then
    ok "commit is reachable from master"
else
    fail "commit '$commit' is not an ancestor of master (or origin/master) — refusing to release from a side branch"
fi

# --- Working tree matches the commit under test --------------------------

# The gates below (Cargo.lock, docs, audit) run against the checked-out
# worktree, so they are only meaningful for the commit actually checked
# out. Fail loudly rather than silently validating the wrong tree.
head_sha=$(git rev-parse HEAD)
commit_sha=$(git rev-parse "$commit^{commit}" 2>/dev/null || echo "")
if [[ "$commit_sha" != "$head_sha" ]]; then
    fail "commit '$commit' ($commit_sha) is not checked out (HEAD is $head_sha); check out the tag commit before running this script"
else
    ok "commit-ish resolves to the currently checked-out HEAD"
fi

if [[ -n "$(git status --porcelain)" ]]; then
    fail "working tree is not clean; a release must be cut from a clean, reproducible tree"
else
    ok "working tree is clean"
fi

# --- Cargo.lock is current -------------------------------------------------

if cargo metadata --locked --format-version 1 --no-deps >/dev/null 2>&1; then
    ok "Cargo.lock is current"
else
    fail "Cargo.lock is out of date relative to Cargo.toml (run 'cargo update' and commit it)"
fi

# --- Generated docs are current -------------------------------------------

if cargo run --locked --package xtask -- docs --check >/dev/null 2>&1; then
    ok "generated operation docs are current"
else
    fail "generated operation docs are stale (run 'cargo run -p xtask -- docs')"
fi

if cargo run --locked --example generate_operation_docs -- --check >/dev/null 2>&1; then
    ok "generated operation reference is current"
else
    fail "generated operation reference is stale (run 'cargo run --example generate_operation_docs')"
fi

# --- Registry consistency --------------------------------------------------

if cargo run --locked --package xtask -- check-registry >/dev/null 2>&1; then
    ok "operation registry is current"
else
    fail "operation registry is stale (run 'cargo run -p xtask -- generate-registry')"
fi

# --- Operation audit is green ----------------------------------------------

if cargo run --locked --package xtask -- audit-operations >/dev/null 2>&1; then
    ok "operation audit passes"
else
    fail "operation audit failed — this must be green before any release"
fi

# --- Version consistency across the registry -------------------------------

if cargo run --locked --package xtask -- verify-version >/dev/null 2>&1; then
    ok "version consistency check passes"
else
    fail "version consistency check failed"
fi

# --- Summary ----------------------------------------------------------------

echo
echo "-----------------------------------------------------------------------"
if [[ "$prerelease" == true ]]; then
    echo "Tag class: PRERELEASE (GitHub prerelease + archives + installers only;"
    echo "           no stable AUR/Homebrew/crates.io publish)"
else
    echo "Tag class: STABLE (full distribution)"
fi

if ((${#failed[@]} > 0)); then
    echo "Failed checks (${#failed[@]}):"
    for entry in "${failed[@]}"; do
        echo "  - $entry"
    done
    echo "RELEASE TAG CHECK: FAIL"
    exit 1
fi

echo "RELEASE TAG CHECK: PASS"
