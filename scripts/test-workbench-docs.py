#!/usr/bin/env python3
"""Execute marked handbook examples against a real CLI, in disposable storage."""
import argparse
import base64
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import unittest
from shell_runtime import bash_executable

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--cli", type=Path, default=ROOT / "target/release/rxchef")
args = parser.parse_args()
CLI = args.cli.resolve()
if not CLI.is_file():
    parser.error(f"build the CLI first: {CLI}")


def example(document, name, language):
    source = (ROOT / "docs" / document).read_text()
    matches = re.findall(
        rf"<!-- rxchef-example: {re.escape(name)} -->\s*```{language}\n(.*?)\n```",
        source, re.DOTALL,
    )
    if len(matches) != 1:
        raise AssertionError(f"expected exactly one {name} example in {document}")
    return matches[0]


class HandbookExamples(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="rxchef-docs-")
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.environment = {**os.environ, "RXCHEF_HOME": str(self.directory / "state"),
                            "PATH": str(CLI.parent) + os.pathsep + os.environ["PATH"]}

    def cli(self, *arguments, payload=None, success=True):
        result = subprocess.run(
            [str(CLI), *arguments], input=payload, capture_output=True,
            cwd=self.directory, env=self.environment, timeout=30,
        )
        if success:
            self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        else:
            self.assertNotEqual(result.returncode, 0)
        return result

    def project(self):
        return json.loads(example("workbench/portable-projects.md", "portable-project", "json"))

    def test_portable_project_validates_and_inspects_without_rewriting(self):
        project = self.project()
        path = self.directory / "session.rxproject.json"
        original = json.dumps(project).encode()
        path.write_bytes(original)
        self.cli("project", "validate", str(path))
        inspected = json.loads(self.cli("project", "inspect", str(path)).stdout)
        self.assertEqual(inspected["name"], project["name"])
        self.assertEqual(inspected["graphs"], [])
        self.assertEqual(inspected["settings"]["theme"], "dark")
        self.assertEqual(path.read_bytes(), original)

    def test_future_project_version_is_rejected_without_rewriting(self):
        project = self.project()
        project["version"] = 2
        path = self.directory / "future.rxproject.json"
        original = json.dumps(project).encode()
        path.write_bytes(original)
        self.cli("project", "validate", str(path), success=False)
        self.assertEqual(path.read_bytes(), original)

    def test_documented_terminal_roundtrip_executes_verbatim(self):
        commands = example("workbench/terminal.md", "terminal-roundtrip", "sh")
        # The handbook assumes repository-root example paths. Link just those
        # fixtures into a disposable cwd, so documented output files stay local.
        (self.directory / "examples").symlink_to(ROOT / "examples", target_is_directory=True)
        result = subprocess.run(
            [bash_executable(), "-euo", "pipefail", "-c", commands], cwd=self.directory,
            env=self.environment, capture_output=True, timeout=30,
        )
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        self.assertEqual((self.directory / "original.bin").read_bytes(), bytes(range(256)))

    def test_selected_output_needs_only_its_named_dependencies(self):
        source = bytes(range(256)) + b"\x00\xff\xc0"
        (self.directory / "payload.bin").write_bytes(source)
        result = self.cli("graph", "run", "--graph", str(ROOT / "examples/graphs/three-inputs.rxgraph.json"),
                          "--input", "payload=payload.bin", "--select", "original")
        self.assertEqual(result.stdout, source)

    def test_documented_recipe_distinguishes_transport_and_operation_base64(self):
        request = json.loads(example("server/protocol.md", "recipe-request", "json"))
        self.assertEqual(base64.b64decode(request["input_base64"], validate=True), b"\x00\xff\x80")
        # Stdio calls the same recipe engine through its own documented method.
        envelope = {"jsonrpc": "2.0", "id": 1, "method": "bake", "params": request}
        response = json.loads(self.cli("serve", "--stdio", payload=(json.dumps(envelope) + "\n").encode()).stdout)
        output = base64.b64decode(response["result"]["output_base64"], validate=True)
        self.assertEqual(output, b"AP+A")
        self.assertEqual(base64.b64decode(output, validate=True), b"\x00\xff\x80")


if __name__ == "__main__":
    unittest.main(argv=[__file__], verbosity=2)
