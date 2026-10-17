#!/usr/bin/env python3
"""Exercise the real recipe editor in a POSIX PTY, using a release TUI binary.

Uses the graph smoke's screen decoder, not a mocked engine. All actions await
observable UI/file/process state with deadlines. Recipe mode currently exposes
no byte-output file export binding; that limitation is reported explicitly.
"""
import argparse
import codecs
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import pty
import select
import subprocess
import tempfile
import termios
import time
import traceback


spec = importlib.util.spec_from_file_location(
    'rxchef_graph_pty', Path(__file__).with_name('tui-graph-smoke.py'))
graph_pty = importlib.util.module_from_spec(spec)
spec.loader.exec_module(graph_pty)


class RecipeTerminal(graph_pty.Terminal):
    def __init__(self, binary, directory):
        self.master, self.slave = pty.openpty()
        self.initial = termios.tcgetattr(self.slave)
        self.decoder = codecs.getincrementaldecoder('utf-8')(errors='replace')
        self.escape = ''
        self.x = self.y = 0
        self.buffer = b''
        self.closed = False
        self.resize(160, 46)
        env = {**os.environ, 'TERM': 'xterm-256color',
               'RXCHEF_HOME': str(directory / 'store')}
        env.pop('RXCHEF_SERVER', None)
        env.pop('RXCHEF_TOKEN', None)
        self.process = subprocess.Popen([str(binary)], stdin=self.slave,
                                        stdout=self.slave, stderr=self.slave,
                                        cwd=directory, env=env)

    def until(self, predicate, description, timeout=20):
        deadline = time.monotonic() + timeout
        while not predicate():
            if self.process.poll() is not None:
                raise AssertionError(f'TUI exited with {self.process.returncode}: {description}')
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise AssertionError(f'Timed out awaiting {description}:\n{self.visible()}')
            if select.select([self.master], [], [], min(0.1, remaining))[0]:
                chunk = os.read(self.master, 65536)
                if not chunk:
                    raise AssertionError(f'Terminal closed awaiting {description}')
                self.buffer += chunk
                self.feed(chunk)
                if b'\x1b[6n' in chunk:
                    os.write(self.master, b'\x1b[1;1R')

    def expect(self, text, timeout=20):
        self.until(lambda: text in self.visible(), repr(text), timeout)

    def output(self):
        # Exclude the left input pane: input and transformed output may contain
        # the same text, so a whole-screen search would be a false positive.
        start = self.width * 45 // 100
        return '\n'.join(''.join(row[start:]) for row in self.cells[:-4])

    def expect_output(self, text):
        self.until(lambda: text in self.output(), f'output pane containing {text!r}')

    def close(self, control_c=False):
        super().close(control_c)
        self.closed = True

    def cleanup(self):
        if self.closed:
            return
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=5)
        os.close(self.master)
        os.close(self.slave)
        self.closed = True


def flow(binary, directory, report):
    screens = report['screens']
    cases = report['cases']
    terminal = RecipeTerminal(binary, directory)

    def passed(name):
        cases.append({'name': name, 'status': 'PASS'})
        screens[name] = terminal.visible()

    try:
        terminal.expect('Welcome!')
        terminal.send('?')
        terminal.expect('NORMAL MODE')
        terminal.send('q')
        terminal.expect('Pipeline (0 steps) [Local]')
        passed('help-open-and-close')

        for operation in ('From Hex', 'To Hex'):
            terminal.send('a')
            terminal.expect('Add step — search operations')
            terminal.send(operation + '\r')
            terminal.expect(f"Added '{operation}'.")
        terminal.expect('Pipeline (2 steps) [Local]')
        passed('create-two-step-recipe-by-operation-search')

        terminal.send('e')
        terminal.expect('Edit args: To Hex')
        terminal.send('\x7f' * len('Space') + 'None\t\x7f0\r')
        terminal.expect("Updated args for 'To Hex'.")
        terminal.send('i00ff8041420a\r')
        terminal.expect('Input updated. Press r to run.')
        terminal.send('r')
        terminal.expect('Pipeline ran OK')
        terminal.expect_output('00ff8041420a')
        passed('edit-arguments-and-execute-real-binary-roundtrip')

        # Two captured actions plus final output: left twice selects From Hex.
        terminal.send('\x1b[D\x1b[Dx')
        terminal.expect('recipe step 1: From Hex')
        terminal.expect_output('00000000  00 ff 80 41 42 0a')
        passed('navigate-to-binary-intermediate-and-inspect-exact-hex')
        terminal.send('\x1b[C\x1b[Cx')
        terminal.expect('Output: Final output')
        terminal.expect_output('00ff8041420a')
        passed('navigate-back-to-final-text-output')

        # Edit again and demand a different output so stale results cannot pass.
        terminal.send('e')
        terminal.expect('Edit args: To Hex')
        terminal.send('\x7f' * len('None') + 'Colon\r')
        terminal.expect("Updated args for 'To Hex'.")
        terminal.send('r')
        terminal.expect_output('00:ff:80:41:42:0a')
        terminal.expect('Pipeline ran OK')
        terminal.send('spty-recipe\r')
        terminal.expect("Saved 'pty-recipe'")
        recipe_path = directory / 'store/recipes/pty-recipe.json'
        recipe = json.loads(recipe_path.read_text())
        assert [step['op'] for step in recipe['steps']] == ['From Hex', 'To Hex'], recipe
        assert recipe['steps'][1]['args'] == ['Colon', '0'], recipe
        report['saved_recipe'] = recipe
        passed('save-edited-recipe-and-validate-written-json')

        terminal.send('d')
        terminal.expect("Deleted 'To Hex'.")
        terminal.send('l')
        terminal.expect('Load recipe (1)')
        terminal.send('\r')
        terminal.expect("Loaded 'pty-recipe' (2 steps).")
        terminal.send('r')
        terminal.expect_output('00:ff:80:41:42:0a')
        passed('delete-step-reload-from-store-and-execute')

        # Verify pipeline selection via j/k by opening the selected step editor.
        terminal.send('j' + 'e')
        terminal.expect('Edit args: To Hex')
        terminal.send('\x1b')
        terminal.expect('Pipeline (2 steps) [Local]')
        terminal.send('k' + 'e')
        terminal.expect('Edit args: From Hex')
        terminal.send('\x1b')
        terminal.expect('Pipeline (2 steps) [Local]')
        passed('keyboard-pipeline-navigation')

        terminal.send('?')
        terminal.expect('NORMAL MODE')
        terminal.resize(20, 8)
        terminal.expect('Help')
        passed('small-terminal-20x8-renders-help-without-crash')
        terminal.resize(160, 46)
        terminal.expect('NORMAL MODE')
        terminal.send('q')
        terminal.expect('Pipeline (2 steps) [Local]')
        terminal.close()
        cases.append({'name': 'normal-quit-restores-terminal-flags', 'status': 'PASS'})
    finally:
        terminal.cleanup()

    restarted = RecipeTerminal(binary, directory)
    try:
        restarted.expect('Welcome!')
        restarted.send('l')
        restarted.expect('Load recipe (1)')
        restarted.send('\r')
        restarted.expect("Loaded 'pty-recipe' (2 steps).")
        restarted.send('i00ff8041420a\r')
        restarted.expect('Input updated. Press r to run.')
        restarted.send('r')
        restarted.expect_output('00:ff:80:41:42:0a')
        screens['restart-load-execute'] = restarted.visible()
        restarted.close(control_c=True)
        cases.append({'name': 'restart-load-execute-and-ctrl-c-restores-terminal',
                      'status': 'PASS'})
    finally:
        restarted.cleanup()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path, help='Actual release/installed rxchef_tui binary')
    parser.add_argument('--report', type=Path, help='Write JSON evidence including reconstructed screens')
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    report = {'binary': str(binary), 'sha256': digest, 'status': 'FAIL',
              'cases': [], 'screens': {}, 'limitations': [
                  'Recipe mode has no byte-output file export binding; exact file export is covered by the separate graph PTY smoke.',
                  'This POSIX PTY test does not certify Windows native console behavior or installer lifecycle.',
                  'Binary input is produced by From Hex; recipe input editor accepts text only.']}
    try:
        with tempfile.TemporaryDirectory(prefix='rxchef-recipe-pty-') as temporary:
            flow(binary, Path(temporary), report)
        assert hashlib.sha256(binary.read_bytes()).hexdigest() == digest, 'TUI binary changed during validation'
        report['status'] = 'PASS'
    except Exception:
        report['error'] = traceback.format_exc()
        raise
    finally:
        if args.report:
            args.report.parent.mkdir(parents=True, exist_ok=True)
            args.report.write_text(json.dumps(report, indent=2, ensure_ascii=False) + '\n')
    print(f"PASS recipe editor: {len(report['cases'])} real PTY scenarios; binary SHA-256 {digest}")
    for limitation in report['limitations']:
        print('LIMITATION: ' + limitation)


if __name__ == '__main__':
    main()
