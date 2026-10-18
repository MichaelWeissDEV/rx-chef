#!/usr/bin/env python3
"""Packaging regression tests with real tar and isolated build-command fixtures."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import unittest

from shell_runtime import bash_executable, bash_path

ROOT = Path(__file__).resolve().parent.parent


class DistributionTests(unittest.TestCase):
    def test_native_windows_paths_are_git_bash_paths(self):
        self.assertEqual(bash_path(r"C:\Users\runner\tmp dir", platform="nt"),
                         "/c/Users/runner/tmp dir")

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="rxchef-packaging-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        (self.root / "scripts").mkdir()
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.output = self.root / "distributions"
        self.output.mkdir()
        self.target = self.root / "target/release"
        self.target.mkdir(parents=True)
        shutil.copy(ROOT / "scripts/build-distributions.sh", self.root / "scripts")
        (self.root / "LICENSE").write_text("fixture license\n")
        for name in ("rxchef", "rxchef_tui", "rxchef-server", "rxchef.exe", "rxchef_tui.exe",
                     "rxchef-server.exe", "librxchef.a", "librxchef.dylib", "librxchef.so",
                     "librxchef.rlib", "rxchef.dll", "rxchef.lib"):
            (self.target / name).write_bytes(b"fixture " + name.encode())
        self.command("cargo", """if [[ "$1" == metadata ]]; then
  echo '{"packages":[{"name":"rxchef","version":"8.9.0-rc.2"}]}'
else
  exit "${BUILD_FAILURE:-0}"
fi
""")
        self.command("npm", "exit 0\n")
        if os.name == "nt":
            # Query Git Bash's own POSIX search path. Python's Windows PATH is
            # semicolon-delimited and cannot be passed to Bash unchanged.
            base_path = subprocess.check_output(
                [bash_executable(), "-c", 'printf %s "$PATH"'], text=True)
            bash_search_path = bash_path(self.bin) + ":" + base_path
        else:
            bash_search_path = str(self.bin) + os.pathsep + os.environ["PATH"]
        self.env = {**os.environ, "PATH": bash_search_path,
                    "CARGO_TARGET_DIR": bash_path(self.target.parent),
                    "RXCHEF_DISTRIBUTION_DIR": bash_path(self.output)}

    def command(self, name, body):
        path = self.bin / name
        path.write_text("#!/usr/bin/env bash\nset -eu\n" + body)
        path.chmod(0o755)

    def run_build(self, variant="cli", success=True):
        result = subprocess.run([bash_executable(), str(self.root / "scripts/build-distributions.sh"), variant],
                                env=self.env, text=True, capture_output=True, timeout=30)
        if success:
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertFalse(list(self.output.glob(".stage-*")), "staging directory leaked")
        return result

    def test_exact_members_checksums_and_no_overwrite(self):
        suffix = ".exe" if os.name == "nt" else ""
        for variant, members in (("cli", {"rxchef"}), ("tui", {"rxchef_tui"}),
                                 ("combined", {"rxchef", "rxchef_tui"}),
                                 ("headless", {"rxchef-server"}), ("web", {"rxchef-server"})):
            with self.subTest(variant=variant):
                self.run_build(variant)
                archive, = self.output.glob(f"rxchef-8.9.0-rc.2-{variant}-*.tar.gz")
                before = archive.read_bytes()
                with tarfile.open(archive) as source:
                    self.assertEqual(set(source.getnames()), {name + suffix for name in members} | {"LICENSE"})
                    self.assertTrue(all(member.isfile() for member in source.getmembers()))
                checksum = Path(str(archive) + ".sha256")
                self.assertEqual(checksum.read_text(), f"{hashlib.sha256(before).hexdigest()}  {archive.name}\n")
                self.run_build(variant, success=False)
                self.assertEqual(archive.read_bytes(), before)

    def test_failed_build_leaves_no_artifacts(self):
        self.env["BUILD_FAILURE"] = "1"
        self.run_build(success=False)
        self.assertEqual(list(self.output.iterdir()), [])

    def test_failed_tar_leaves_no_partial_artifacts(self):
        if os.name == "nt":
            # Git Bash resolves the native tar.exe before an extensionless
            # fixture command. Make the real GNU tar fail via its own option.
            self.env["TAR_OPTIONS"] = "--rxchef-intentional-tar-failure"
        else:
            self.command("tar", 'printf partial > "$2"\nexit 1\n')
        self.run_build(success=False)
        self.assertEqual(list(self.output.iterdir()), [])

    def test_existing_checksum_is_not_overwritten(self):
        self.run_build()
        archive, = self.output.glob("*.tar.gz")
        archive.unlink()
        checksum, = self.output.glob("*.sha256")
        checksum.write_text("existing checksum")
        self.run_build(success=False)
        self.assertEqual(checksum.read_text(), "existing checksum")
        self.assertFalse(archive.exists())

    def test_existing_dangling_symlink_is_not_overwritten(self):
        self.run_build()
        archive, = self.output.glob("*.tar.gz")
        archive.unlink()
        Path(str(archive) + ".sha256").unlink()
        try:
            archive.symlink_to("absent")
        except OSError:
            self.skipTest("symlink creation unavailable on this host")
        self.run_build(success=False)
        self.assertTrue(archive.is_symlink())
        self.assertFalse(Path(str(archive) + ".sha256").exists())

    def test_archive_publication_race_rolls_back_only_our_checksum(self):
        python = bash_path(shutil.which("python3"))
        # Introduce a competing archive after the shell's initial existence check.
        self.command("python3", f'''if [[ "$1" == - ]]; then
  printf competitor > "$3"
fi
exec "{python}" "$@"
''')
        self.run_build(success=False)
        archive, = self.output.glob("*.tar.gz")
        self.assertEqual(archive.read_bytes(), b"competitor")
        self.assertFalse(Path(str(archive) + ".sha256").exists())

    def test_windows_git_bash_names(self):
        # This checks platform naming only; native Git Bash/tar remains a CI gate.
        self.command("uname", 'if [[ "$1" == -s ]]; then echo MINGW64_NT-10.0; else echo x86_64; fi\n')
        self.run_build("combined")
        archive = self.output / "rxchef-8.9.0-rc.2-combined-Windows-x86_64.tar.gz"
        with tarfile.open(archive) as source:
            self.assertEqual(set(source.getnames()), {"LICENSE", "rxchef.exe", "rxchef_tui.exe"})

    def test_smoke_version_comes_from_metadata(self):
        shutil.copy(ROOT / "scripts/smoke-distributions.py", self.root / "scripts")
        (self.root / "Cargo.toml").write_text(
            '[package]\nname = "rxchef"\nversion = "8.9.0-rc.2"\nedition = "2021"\n')
        source = self.root / "src"
        source.mkdir()
        (source / "lib.rs").write_text("")
        (self.root / "Cargo.lock").write_text(
            'version = 4\n\n[[package]]\nname = "rxchef"\nversion = "8.9.0-rc.2"\n')
        examples = self.root / "examples/graphs"
        examples.mkdir(parents=True)
        (examples / "three-inputs.rxgraph.json").write_text("{}")
        archive = self.output / "rxchef-8.9.0-rc.2-library-Test.tar.gz"
        with tarfile.open(archive, "w:gz") as destination:
            destination.add(self.root / "LICENSE", arcname="LICENSE")
        Path(str(archive) + ".sha256").write_text(hashlib.sha256(archive.read_bytes()).hexdigest())
        # This fixture is Python-launched and has a tiny real Cargo package.
        # Use the native PATH so metadata is verified by real cargo on every OS.
        env = {**self.env, "PATH": os.environ["PATH"]}
        result = subprocess.run([shutil.which("python3"), str(self.root / "scripts/smoke-distributions.py"),
                                 "--directory", str(self.output)], env=env,
                                capture_output=True, text=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(archive.name, result.stdout)
        self.assertEqual(json.loads((self.output / "installation-smoke.json").read_text())[0]["smoke"], "PASS")

    def homebrew_fixture(self, binary, smoke_fails=False):
        shutil.copy(ROOT / "scripts/test-homebrew-formula.sh", self.root / "scripts")
        keg = self.root / "keg/bin"
        keg.mkdir(parents=True)
        installed = keg / binary
        installed.write_text("#!/usr/bin/env bash\nset -eu\n" + (
            'if [[ "$1" == --version ]]; then echo rxchef-8.9.0; '
            'elif [[ "$1" == run && "$2" == "To Base64" ]]; then '
            'read -r data || :; [[ "$data" == hello ]]; echo aGVsbG8=; else exit 9; fi\n'
            if binary == "rxchef" else
            'if [[ "$1" == --help ]]; then echo "--graph FILE"; else exit 9; fi\n'))
        installed.chmod(0o755)
        log = self.root / "brew.log"
        self.command("brew", f'''echo "$*" >> "{log}"
if [[ "$1" == --prefix ]]; then echo "{keg.parent}"; fi
''')
        (self.root / "scripts/tui-graph-smoke.py").write_text(
            "import pathlib, sys\n"
            f"assert pathlib.Path(sys.argv[1]) == pathlib.Path({str(installed)!r})\n"
            f"pathlib.Path({str(self.root / 'pty-invoked')!r}).touch()\n"
            + ("raise SystemExit(1)\n" if smoke_fails else ""))
        result = subprocess.run([bash_executable(), str(self.root / "scripts/test-homebrew-formula.sh"),
                                 str(self.root / "rxchef-test.rb"), binary], env=self.env,
                                capture_output=True, text=True, timeout=30)
        return result, log.read_text()

    def test_homebrew_cli_uses_installed_keg(self):
        result, log = self.homebrew_fixture("rxchef")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(log.count("uninstall rxchef-test"), 1)
        self.assertFalse((self.root / "pty-invoked").exists())

    def test_homebrew_tui_delegates_installed_binary_to_pty(self):
        result, log = self.homebrew_fixture("rxchef_tui")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue((self.root / "pty-invoked").is_file())
        self.assertEqual(log.count("uninstall rxchef-test"), 1)

    def test_homebrew_failed_smoke_still_uninstalls(self):
        result, log = self.homebrew_fixture("rxchef_tui", smoke_fails=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(log.count("uninstall rxchef-test"), 1)


if __name__ == "__main__":
    unittest.main()
