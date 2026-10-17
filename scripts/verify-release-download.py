#!/usr/bin/env python3
"""Compare downloaded release assets with the exact files supplied for upload."""
import argparse
import hashlib
from pathlib import Path


def inventory(directory):
    files = {}
    for path in directory.iterdir():
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"Expected a flat directory of regular assets: {path}")
        digest = hashlib.sha256()
        with path.open('rb') as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                digest.update(chunk)
        files[path.name] = (path.stat().st_size, digest.hexdigest())
    if not files:
        raise ValueError(f"No release assets found: {directory}")
    return files


def verify(expected, downloaded):
    originals = inventory(expected)
    actual = inventory(downloaded)
    if originals.keys() != actual.keys():
        raise ValueError(f"Asset names differ: missing={sorted(originals.keys() - actual.keys())}, "
                         f"unexpected={sorted(actual.keys() - originals.keys())}")
    for name, metadata in originals.items():
        if actual[name] != metadata:
            raise ValueError(f"Downloaded asset has different size/SHA-256: {name}")
    return len(originals)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('expected', type=Path)
    parser.add_argument('downloaded', type=Path)
    args = parser.parse_args()
    try:
        count = verify(args.expected, args.downloaded)
    except (OSError, ValueError) as error:
        parser.exit(1, f'RELEASE DOWNLOAD: FAIL — {error}\n')
    print(f'RELEASE DOWNLOAD: PASS — {count} exact assets verified')
