#!/usr/bin/env python3
"""Local release version regression tests; creates no tags or publications."""
import importlib.util
import pathlib
import os
import shutil
import subprocess
import tempfile
import unittest
import json
import re
import shlex
import sys
from shell_runtime import bash_executable
from unittest import mock

spec = importlib.util.spec_from_file_location("release_version", pathlib.Path(__file__).with_name("release-version.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ReleaseVersionTests(unittest.TestCase):
    def test_release_matrix(self):
        cases = [
            ("v0.0.1", "0.0.1", True),
            ("v0.2.0-rc.1", "0.2.0-rc.1", True),
            ("v0.2.0-rc.2", "0.2.0-rc.1", False),
            ("v0.2.0", "0.2.0-rc.1", False),
            ("v0.2.0", "0.2.0", True),
            ("v0.2.0-beta.1", "0.2.0-beta.1", True),
            ("v0.2.0-invalid", "0.2.0", False),
            ("version0.2.0", "0.2.0", False),
        ]
        for tag, version, valid in cases:
            with self.subTest(tag=tag, version=version):
                if valid:
                    self.assertEqual(module.validate(tag, version, f"## [{version}] - date\n")[0], version)
                else:
                    with self.assertRaises(ValueError):
                        module.validate(tag, version, f"## [{version}]\n")

    def test_publish_order_respects_workspace_runtime_dependencies(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        workflow = (root / '.github/workflows/publish-crates.yml').read_text()
        order = re.findall(r'^\s+run: ./scripts/publish-crate.sh ([a-zA-Z0-9_-]+)\s*$', workflow, re.MULTILINE)
        self.assertTrue(order)
        self.assertEqual(len(order), len(set(order)))
        metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--no-deps', '--format-version', '1'], cwd=root))
        packages = {package['name']: package for package in metadata['packages']}
        for name in order:
            self.assertIn(name, packages)
            for dependency in packages[name]['dependencies']:
                if dependency['name'] in order and dependency.get('kind') != 'dev':
                    with self.subTest(package=name, dependency=dependency['name']):
                        self.assertLess(order.index(dependency['name']), order.index(name))

    def test_native_windows_bash_avoids_wsl(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            git = root / 'Git/cmd/git.exe'; git.parent.mkdir(parents=True); git.touch()
            bash = root / 'Git/bin/bash.exe'; bash.parent.mkdir(parents=True); bash.touch()
            wsl = root / 'Windows/System32/bash.exe'; wsl.parent.mkdir(parents=True); wsl.touch()
            with mock.patch('shell_runtime.shutil.which', side_effect=lambda name: str(git if name == 'git' else wsl)):
                self.assertEqual(pathlib.Path(bash_executable(platform='nt')), bash.resolve())
            with mock.patch('shell_runtime.shutil.which', return_value=None), mock.patch.dict(os.environ, {'ProgramFiles': ''}):
                with self.assertRaisesRegex(RuntimeError, 'Git Bash'):
                    bash_executable(platform='nt')

    def test_custom_publish_caller_grants_reusable_workflow_permissions(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        generated = (root / '.github/workflows/release.yml').read_text()
        caller = generated.split('  custom-publish-crates:\n', 1)[1].split('\n  announce:', 1)[0]
        called = (root / '.github/workflows/publish-crates.yml').read_text()
        pattern = r'^\s+["\']?([a-z-]+)["\']?: ["\']?(read|write)["\']?\s*$'
        grants = dict(re.findall(pattern, caller, re.MULTILINE))
        requires = dict(re.findall(pattern, called, re.MULTILINE))
        levels = {'none': 0, 'read': 1, 'write': 2}
        for scope, permission in requires.items():
            with self.subTest(scope=scope):
                self.assertGreaterEqual(levels[grants.get(scope, 'none')], levels[permission])

    def test_publication_rejects_skipped_failed_and_cancelled_gates(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        workflow = (root / '.github/workflows/release.yml').read_text()
        host = workflow.split('  host:\n', 1)[1].split('  publish-homebrew-formula:', 1)[0]
        dependencies = re.findall(r'^      - ([a-z-]+)$', host, re.MULTILINE)
        required = {'plan', 'custom-validate-tag', 'custom-release-gates', 'build-local-artifacts',
                    'build-global-artifacts', 'custom-build-deb', 'custom-build-rpm',
                    'custom-build-aur', 'custom-build-workbench'}
        self.assertTrue(required.issubset(dependencies))
        condition = next(line for line in host.splitlines() if line.startswith('    if:'))
        for result in ['failure', 'cancelled', 'skipped']:
            self.assertIn("!contains(needs.*.result, '" + result + "')", condition)
        self.assertIn("needs.plan.outputs.publishing == 'true'", condition)

    def test_tag_validation_installs_registry_formatter(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        workflow = (root / '.github/workflows/validate-tag.yml').read_text()
        self.assertRegex(workflow, r'toolchain install [^\n]+--component rustfmt')

    def test_windows_archive_build_uses_native_perl(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        for name in ['release.yml', 'platform-checks.yml', 'build-workbench.yml']:
            workflow = (root / '.github/workflows' / name).read_text()
            with self.subTest(workflow=name):
                self.assertIn('OPENSSL_SRC_PERL=C:/Strawberry/perl/bin/perl.exe', workflow)
                self.assertIn('-MLocale::Maketext::Simple -MIPC::Cmd', workflow)

    def test_invalid_identifiers(self):
        for version in ["01.2.0", "0.02.0", "0.2.00", "0.2.0-01", "0.2.0-rc.01", "0.2.0-", "0.2.0-rc..1", "0.2.0-rc_1", "0.2.0+", "0.2.0+a..b", "0.2.0\n"]:
            with self.subTest(version=version), self.assertRaises(ValueError):
                module.validate("v" + version, version, f"## [{version}]\n")

    def test_build_metadata_and_classification(self):
        for version, prerelease in [("0.2.0-rc.1+build.001", "rc.1"), ("0.2.0+build-1", None)]:
            with self.subTest(version=version):
                self.assertEqual(module.validate("v" + version, version, f"## [{version}]\n"), (version, "0.2.0", prerelease))
        with self.assertRaises(ValueError):
            module.validate("v0.2.0+build.1", "0.2.0+build.2", "## [0.2.0+build.1]\n")

    def test_exact_changelog_heading(self):
        for heading in ["## [0.2.0]", "## [0x2x0-rcx1]", "## [0.2.0-rc.1]extra", "### [0.2.0-rc.1]", "## [0.2.0-rc.1+build]"]:
            with self.subTest(heading=heading), self.assertRaises(ValueError):
                module.validate("v0.2.0-rc.1", "0.2.0-rc.1", heading)

    def test_full_gate_on_clean_synthetic_checkout_without_creating_tags(self):
        source = pathlib.Path(__file__).parent
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "scripts").mkdir()
            for name in ["check-release-tag.sh", "release-version.py"]:
                shutil.copy(source / name, root / "scripts" / name)
            (root / "CHANGELOG.md").write_text("## [0.2.0-rc.1]\n")
            (root / "bin").mkdir()
            cargo = root / "bin" / "cargo"
            program = 'import json,sys\nif sys.argv[1] == "metadata": print(json.dumps({"packages":[{"name":"rxchef","version":"0.2.0-rc.1"}]}))'
            cargo.write_text('#!/bin/sh\nexec ' + shlex.quote(sys.executable.replace('\\', '/')) + ' -c ' + shlex.quote(program) + ' "$@"\n', newline='\n')
            cargo.chmod(0o755)
            def git(*args):
                subprocess.run(["git", *args], cwd=root, check=True, capture_output=True)
            git("init", "--initial-branch=master")
            git("add", ".")
            git("-c", "user.name=Release test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "commit", "-m", "fixture")
            result = subprocess.run([bash_executable(), "scripts/check-release-tag.sh", "v0.2.0-rc.1"], cwd=root,
                                    env={**os.environ, "PATH": str(root / "bin") + os.pathsep + os.environ["PATH"]}, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("PRERELEASE", result.stdout)
            self.assertIn("RELEASE TAG CHECK: PASS", result.stdout)
            self.assertEqual(subprocess.check_output(["git", "tag", "--list"], cwd=root), b"")

    def test_stable_publish_jobs_keep_prerelease_guard(self):
        root = pathlib.Path(__file__).resolve().parent.parent
        workflow = (root / ".github/workflows/release.yml").read_text()
        for job in ["publish-homebrew-formula", "custom-publish-crates"]:
            block = workflow.split("  " + job + ":\n", 1)[1]
            self.assertIn("!fromJson(needs.plan.outputs.val).announcement_is_prerelease", block.split("\n\n", 1)[0])
        config = (root / "dist-workspace.toml").read_text()
        self.assertNotIn("publish-prereleases = true", config)
        reusable = (root / ".github/workflows/publish-crates.yml").read_text()
        self.assertIn("!fromJson(inputs.plan).announcement_is_prerelease", reusable)
        self.assertIn("fromJson(inputs.plan).announcement_tag == github.ref_name", reusable)


if __name__ == "__main__":
    unittest.main()
