#!/usr/bin/env python3
"""Validate a release tag against the exact Cargo and changelog version."""
import argparse
import pathlib
import re

CORE = r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
IDENTIFIER = r"(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)"
SEMVER = re.compile(r"^v" + CORE + r"(?:-(" + IDENTIFIER + r"(?:\." + IDENTIFIER + r")*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$")


def validate(tag, workspace_version, changelog):
    match = SEMVER.fullmatch(tag)
    if not match:
        raise ValueError("tag must be v<major>.<minor>.<patch>[-<prerelease>][+<build>] with valid SemVer identifiers")
    full_version = tag[1:]
    core = ".".join(match.group(i) for i in (1, 2, 3))
    prerelease = match.group(4)
    if workspace_version != full_version:
        raise ValueError(f"tag version '{full_version}' != Cargo workspace version '{workspace_version}'")
    heading = re.compile(r"^## \[" + re.escape(full_version) + r"\](?:\s.*)?$", re.MULTILINE)
    if not heading.search(changelog):
        raise ValueError(f"CHANGELOG.md has no exact [{full_version}] section")
    return full_version, core, prerelease


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tag")
    parser.add_argument("workspace_version")
    parser.add_argument("changelog", type=pathlib.Path)
    args = parser.parse_args()
    try:
        full, core, prerelease = validate(args.tag, args.workspace_version, args.changelog.read_text())
    except ValueError as error:
        parser.exit(1, str(error) + "\n")
    print(f"{full} (core {core}; {'PRERELEASE' if prerelease else 'STABLE'})")
