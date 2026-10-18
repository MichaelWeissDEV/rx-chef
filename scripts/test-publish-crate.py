#!/usr/bin/env python3
"""No-publication tests for registry probes, dry-runs and propagation gates."""
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import tempfile
import unittest

from shell_runtime import bash_executable, bash_path

ROOT = Path(__file__).resolve().parent.parent


class PublishCrateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="rxchef-publish-test-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "scripts").mkdir()
        self.bin = self.root / "bin"
        self.bin.mkdir()
        shutil.copy(ROOT / "scripts/publish-crate.sh", self.root / "scripts")
        self.log = self.root / "calls"
        if os.name == "nt":
            # Python inherits Windows' semicolon-delimited PATH. Git Bash only
            # searches colon-delimited POSIX paths, so preserve Bash's own
            # runtime PATH and prepend the converted fixture directory.
            base_path = subprocess.check_output(
                [bash_executable(), "-c", 'printf %s "$PATH"'], text=True)
            bash_search_path = bash_path(self.bin) + ":" + base_path
        else:
            bash_search_path = str(self.bin) + os.pathsep + os.environ["PATH"]
        self.env = {**os.environ, "PATH": bash_search_path,
                    "CALL_LOG": bash_path(self.log), "CARGO_REGISTRY_TOKEN": "fixture-token",
                    "INDEX_CALLS": bash_path(self.root / "index-count"), "REGISTRY_CASE": "publish",
                    # The Bash helpers need POSIX paths; the native Python
                    # curl fixture needs paths in its host filesystem form.
                    "CALL_LOG_NATIVE": str(self.log),
                    "INDEX_CALLS_NATIVE": str(self.root / "index-count")}
        self.command("cargo", '''if [[ "$1" == metadata ]]; then
  echo '{"packages":[{"name":"rxchef","version":"0.2.0"}]}'
else
  echo "cargo $*" >> "$CALL_LOG"
  exit "${CARGO_FAILURE:-0}"
fi
''')
        self.command("sleep", 'echo "sleep $*" >> "$CALL_LOG"\n')
        curl = self.bin / "curl"
        curl_fixture = self.bin / "curl.py"
        python = bash_path(shutil.which("python3"))
        # Invoke the native interpreter explicitly from a Bash fixture. Git
        # Bash does not reliably honour a native Windows Python shebang on a
        # script found through PATH, which would otherwise fall through to its
        # bundled real curl.exe and make the tests wait for network timeouts.
        curl.write_text(
            "#!/usr/bin/env bash\nset -eu\nexec " + shlex.quote(python)
            + ' "$(dirname "$0")/curl.py" "$@"\n'
        )
        curl_fixture.write_text('''import json, os, pathlib, sys
args = sys.argv[1:]
url = args[-1]
path = pathlib.Path(args[args.index("--output") + 1])
case = os.environ["REGISTRY_CASE"]
with open(os.environ["CALL_LOG_NATIVE"], "a") as log:
    log.write("curl " + url + "\\n")
if case == "network":
    sys.exit(6)
if case == "http500":
    print("500", end=""); sys.exit()
if case == "http429":
    print("429", end=""); sys.exit()
if "crates.io/api/" in url:
    exists = case.startswith("existing")
    path.write_text(json.dumps({"version": {"num": "0.2.0"}}) if case != "existing-invalid" else "invalid")
    print("200" if exists else "404", end="")
else:
    counter = pathlib.Path(os.environ["INDEX_CALLS_NATIVE"])
    count = int(counter.read_text()) + 1 if counter.exists() else 1
    counter.write_text(str(count))
    if case == "index-network":
        sys.exit(28)
    if case == "index-missing":
        print("404", end=""); sys.exit()
    body = {"vers": "0.1.0" if case == "delayed" and count < 3 else "0.2.0", "yanked": case == "yanked"}
    path.write_text("invalid" if case == "index-invalid" else json.dumps(body) + "\\n")
    print("200", end="")
''')
        curl.chmod(0o755)

    def test_native_windows_paths_are_git_bash_paths(self):
        self.assertEqual(bash_path(r"C:\Users\runner\tmp dir", platform="nt"),
                         "/c/Users/runner/tmp dir")

    def test_fixtures_shadow_real_commands(self):
        resolved = subprocess.check_output(
            [bash_executable(), "-c", "command -v cargo curl sleep"],
            env=self.env,
            text=True,
        ).splitlines()
        self.assertEqual([Path(path).name for path in resolved], ["cargo", "curl", "sleep"])
        self.assertTrue(all(path.startswith(bash_path(self.bin) + "/") for path in resolved))

    def command(self, name, body):
        path = self.bin / name
        path.write_text("#!/usr/bin/env bash\nset -eu\n" + body)
        path.chmod(0o755)

    def run_helper(self, *args, case="publish", success=True):
        self.env["REGISTRY_CASE"] = case
        result = subprocess.run([bash_executable(), str(self.root / "scripts/publish-crate.sh"), *args],
                                env=self.env, capture_output=True, text=True, timeout=30)
        if success:
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        return result, self.log.read_text() if self.log.exists() else ""

    def test_publish_waits_for_exact_sparse_version(self):
        result, log = self.run_helper("rxchef", case="delayed")
        self.assertEqual(log.count("cargo publish --locked -p rxchef\n"), 1)
        self.assertEqual(log.count("https://index.crates.io/rx/ch/rxchef"), 3)
        self.assertEqual(log.count("sleep 10"), 2)
        self.assertIn("resolvable", result.stdout)

    def test_existing_version_still_requires_sparse_index(self):
        result, log = self.run_helper("rxchef", case="existing")
        self.assertNotIn("cargo publish", log)
        self.assertIn("https://index.crates.io/rx/ch/rxchef", log)
        self.assertIn("SKIP publish", result.stdout)

    def test_existing_version_dry_run_always_executes_cargo(self):
        _, log = self.run_helper("rxchef", "--dry-run", case="existing")
        self.assertIn("cargo publish --locked -p rxchef --dry-run --allow-dirty", log)
        self.assertNotIn("curl", log)

    def test_dry_run_failure_is_not_hidden(self):
        self.env["CARGO_FAILURE"] = "1"
        self.run_helper("rxchef", "--dry-run", case="existing", success=False)

    def test_unknown_packages_and_arguments_are_rejected(self):
        for arguments in [[], ["rxchef-server"], ["rxchef", "--unknown"],
                          ["rxchef", "--dry-run", "extra"], ["rxchef'()"]]:
            with self.subTest(arguments=arguments):
                _, log = self.run_helper(*arguments, success=False)
                self.assertEqual(log, "")

    def test_ambiguous_registry_failures_never_publish(self):
        for case in ["network", "http500", "http429", "existing-invalid"]:
            with self.subTest(case=case):
                self.log.unlink(missing_ok=True)
                _, log = self.run_helper("rxchef", case=case, success=False)
                self.assertNotIn("cargo publish", log)

    def test_sparse_failures_stop_next_publish(self):
        for case in ["index-network", "index-invalid", "yanked"]:
            with self.subTest(case=case):
                self.log.unlink(missing_ok=True)
                result, log = self.run_helper("rxchef", case=case, success=False)
                self.assertEqual(log.count("cargo publish"), 1)
                self.assertNotIn("resolvable", result.stdout)

    def test_sparse_missing_version_times_out(self):
        result, log = self.run_helper("rxchef", case="index-missing", success=False)
        self.assertEqual(log.count("https://index.crates.io/"), 30)
        self.assertEqual(log.count("sleep 10"), 29)
        self.assertIn("did not reach", result.stderr)

    def test_missing_token_prevents_publish(self):
        del self.env["CARGO_REGISTRY_TOKEN"]
        _, log = self.run_helper("rxchef", success=False)
        self.assertNotIn("cargo publish", log)

    def test_cargo_publish_failure_stops_before_sparse_probe(self):
        self.env["CARGO_FAILURE"] = "1"
        _, log = self.run_helper("rxchef", success=False)
        self.assertNotIn("index.crates.io", log)


if __name__ == "__main__":
    unittest.main()
