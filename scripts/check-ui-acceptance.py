#!/usr/bin/env python3
"""Bind explicit UI acceptance to a version and immutable GUI source. Never publish."""
import argparse
import datetime
import json
from pathlib import Path
import re
import subprocess

UI_PATHS = ['apps/web', 'apps/desktop', 'Cargo.toml', 'Cargo.lock']


def validate(record, version, head, git):
    if record.get('version') != version:
        raise ValueError('UI acceptance belongs to a different version')
    if record.get('status') != 'accepted':
        raise ValueError('UI acceptance is pending: review desktop and browser UI before releasing')
    accepted = record.get('accepted_commit')
    if not isinstance(accepted, str) or not re.fullmatch(r'[0-9a-f]{40}', accepted):
        raise ValueError('accepted_commit must be a full immutable Git commit SHA')
    if not isinstance(record.get('reviewer'), str) or not record['reviewer'].strip():
        raise ValueError('An explicit reviewer is required')
    try:
        reviewed = datetime.datetime.fromisoformat(record['reviewed_at'].replace('Z', '+00:00'))
        if reviewed.tzinfo is None:
            raise ValueError('timezone required')
    except (KeyError, TypeError, ValueError, AttributeError) as error:
        raise ValueError('reviewed_at must be an ISO-8601 time with timezone') from error
    if git('merge-base', '--is-ancestor', accepted, head).returncode:
        raise ValueError('The accepted UI commit is not an ancestor of the release commit')
    before = git('ls-tree', '-r', accepted, '--', *UI_PATHS)
    after = git('ls-tree', '-r', head, '--', *UI_PATHS)
    if before.returncode or after.returncode or not before.stdout or before.stdout != after.stdout:
        raise ValueError('GUI source changed after acceptance; review the new UI commit')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--file', type=Path, default=Path('.release/ui-acceptance.json'))
    parser.add_argument('--allow-pending', action='store_true', help='Rehearsal only; never use for publication')
    args = parser.parse_args()
    record = json.loads(args.file.read_text())
    metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--no-deps', '--format-version', '1']))
    version = next(p['version'] for p in metadata['packages'] if p['name'] == 'rxchef')
    if args.allow_pending and record.get('status') == 'pending' and record.get('version') == version:
        print('UI ACCEPTANCE: PENDING — rehearsal allowed, publication blocked')
        return
    git = lambda *command: subprocess.run(['git', *command], capture_output=True, text=True)
    head = git('rev-parse', 'HEAD').stdout.strip()
    try:
        validate(record, version, head, git)
    except ValueError as error:
        parser.exit(1, f'UI ACCEPTANCE: BLOCKED — {error}\n')
    print(f'UI ACCEPTANCE: PASS — {version}, reviewed commit {record["accepted_commit"]}')


if __name__ == '__main__':
    main()
