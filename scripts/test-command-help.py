#!/usr/bin/env python3
"""Verify shipped CLI/TUI/server help without executing work or leaking tokens."""
import argparse
import os
from pathlib import Path
import subprocess
import unittest

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--directory', type=Path, default=Path('target/release'))
args = parser.parse_args()
suffix = '.exe' if os.name == 'nt' else ''
bin_dir = args.directory.resolve()


class CommandHelp(unittest.TestCase):
    def run_help(self, binary, arguments, expected):
        secret = 'help-test-secret-must-never-be-printed'
        result = subprocess.run([str(bin_dir / (binary + suffix)), *arguments, '--help'],
                                capture_output=True, text=True, timeout=15,
                                env={**os.environ, 'RXCHEF_TOKEN': secret})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(expected, result.stdout)
        self.assertNotIn(secret, result.stdout + result.stderr)

    def test_cli_command_tree(self):
        commands = [[], ['run'], ['pipe'], ['bake'], ['recipe'], ['graph'],
                    ['graph', 'run'], ['graph', 'validate'], ['graph', 'store'],
                    ['project'], ['project', 'validate'], ['history'], ['pipeline'],
                    ['var'], ['serve'], ['doctor'], ['connect'], ['completions'], ['manpage']]
        for command in commands:
            with self.subTest(command=command):
                self.run_help('rxchef', command, 'Usage: rxchef')

    def test_server_limits_authentication_and_host_options_are_discoverable(self):
        for flag in ['--token-file', '--allowed-host', '--trust-tls-terminator', '--memory-mib', '--retain-secs']:
            with self.subTest(flag=flag):
                self.run_help('rxchef-server', [], flag)

    def test_generated_manual_and_completions(self):
        for shell in ['bash', 'zsh', 'fish', 'powershell', 'elvish']:
            with self.subTest(shell=shell):
                result = subprocess.run([str(bin_dir / ('rxchef' + suffix)), 'completions', shell], capture_output=True, timeout=15)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn(b'rxchef', result.stdout)
        result = subprocess.run([str(bin_dir / ('rxchef' + suffix)), 'manpage'], capture_output=True, timeout=15)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(b'.TH', result.stdout)

    def test_tui_graph_and_remote_options_are_discoverable(self):
        for flag in ['--graph', '--server', '--token']:
            with self.subTest(flag=flag):
                self.run_help('rxchef_tui', [], flag)


if __name__ == '__main__':
    unittest.main(argv=[__file__], verbosity=2)
