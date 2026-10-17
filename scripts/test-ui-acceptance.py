#!/usr/bin/env python3
import importlib.util
from pathlib import Path
from types import SimpleNamespace
import unittest
import subprocess
import tempfile
spec = importlib.util.spec_from_file_location('acceptance', Path(__file__).with_name('check-ui-acceptance.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class AcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.record = dict(version='0.2.0-rc.1', status='accepted', accepted_commit='a' * 40, reviewer='Human reviewer', reviewed_at='2026-10-15T18:00:00+02:00')

    def git(self, *args):
        return SimpleNamespace(returncode=0, stdout='' if args[0] == 'status' else '100644 blob gui-source\n')

    def check(self, **changes):
        module.validate({**self.record, **changes}, '0.2.0-rc.1', 'b' * 40, self.git)

    def test_accepted_unchanged_ui_allows_later_documentation_commits(self):
        self.check()

    def test_pending_and_other_versions_fail(self):
        for changes in [dict(status='pending'), dict(version='0.3.0'), dict(accepted_commit='HEAD'), dict(reviewer=''), dict(reviewed_at='2026-10-15')]:
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                self.check(**changes)

    def test_unrelated_accepted_commit_fails(self):
        def git(*args):
            return SimpleNamespace(returncode=1 if args[0] == 'merge-base' else 0, stdout='source')
        with self.assertRaisesRegex(ValueError, 'ancestor'):
            module.validate(self.record, '0.2.0-rc.1', 'b' * 40, git)

    def test_real_git_allows_docs_but_rejects_later_gui_and_lockfile_edits(self):
        with tempfile.TemporaryDirectory(prefix='rxchef-ui-acceptance-') as directory:
            root = Path(directory)
            (root / 'hooks').mkdir()
            def git(*args):
                return subprocess.run(['git', '-c', 'core.hooksPath=' + str(root / 'hooks'), '-c', 'commit.gpgsign=false',
                                       '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid', *args],
                                      cwd=root, text=True, capture_output=True)
            def commit():
                self.assertEqual(git('add', '.').returncode, 0)
                self.assertEqual(git('commit', '-m', 'fixture').returncode, 0)
                return git('rev-parse', 'HEAD').stdout.strip()
            self.assertEqual(git('init').returncode, 0)
            source = root / 'apps/web/src/view.ts'; source.parent.mkdir(parents=True)
            source.write_text('original UI')
            lock = root / 'Cargo.lock'; lock.write_text('original lockfile')
            accepted = commit()
            record = {**self.record, 'accepted_commit': accepted}
            module.validate(record, '0.2.0-rc.1', accepted, git)
            (root / 'README.md').write_text('documentation review')
            module.validate(record, '0.2.0-rc.1', commit(), git)
            source.write_text('changed UI')
            with self.assertRaisesRegex(ValueError, 'working tree differs'):
                module.validate(record, '0.2.0-rc.1', git('rev-parse', 'HEAD').stdout.strip(), git)
            with self.assertRaisesRegex(ValueError, 'source changed'):
                module.validate(record, '0.2.0-rc.1', commit(), git)
            source.write_text('original UI'); lock.write_text('changed lockfile')
            with self.assertRaisesRegex(ValueError, 'source changed'):
                module.validate(record, '0.2.0-rc.1', commit(), git)

    def test_changed_or_missing_ui_source_fails(self):
        for missing in [False, True]:
            def git(*args):
                text = '' if missing else 'old' if 'a' * 40 in args else 'new'
                return SimpleNamespace(returncode=0, stdout=text)
            with self.subTest(missing=missing), self.assertRaisesRegex(ValueError, 'source changed'):
                module.validate(self.record, '0.2.0-rc.1', 'b' * 40, git)


if __name__ == '__main__':
    unittest.main()
