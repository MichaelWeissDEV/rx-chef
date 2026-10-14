#!/usr/bin/env python3
"""Real POSIX PTY tests for the installed/built TUI; no mocked engine results."""
import argparse
import codecs
import base64
import errno
import fcntl
import json
import os
from pathlib import Path
import pty
import re
import select
import socket
import struct
import subprocess
import tempfile
import termios
import time
import urllib.request


class Terminal:
    def __init__(self, binary, graph, directory, remote=None):
        self.master, self.slave = pty.openpty()
        self.initial = termios.tcgetattr(self.slave)
        self.decoder = codecs.getincrementaldecoder('utf-8')(errors='replace')
        self.escape = ''
        self.x = self.y = 0
        self.resize(100, 30)
        args = [str(binary), '--graph', str(graph)]
        if remote:
            args += ['--server', remote]
        env = {**os.environ, 'TERM': 'xterm-256color', 'RXCHEF_HOME': str(directory / 'store')}
        env.pop('RXCHEF_SERVER', None)
        env['RXCHEF_TOKEN'] = 'pty-test-token-0123456789'
        self.process = subprocess.Popen(args, stdin=self.slave, stdout=self.slave,
                                        stderr=self.slave, cwd=directory, env=env)
        self.buffer = b''

    def resize(self, width, height):
        self.width, self.height = width, height
        self.cells = [[' '] * width for _ in range(height)]
        fcntl.ioctl(self.slave, termios.TIOCSWINSZ, struct.pack('HHHH', height, width, 0, 0))

    def feed(self, chunk):
        # Ratatui emits cursor-addressed screen differences, so removing ANSI
        # sequences loses spaces and unchanged characters. Reconstruct its cells.
        for char in self.decoder.decode(chunk):
            if self.escape:
                self.escape += char
                if self.escape.startswith('\x1b['):
                    if len(self.escape) < 3 or not ('@' <= char <= '~'):
                        continue
                    command = self.escape[-1]
                    values = [int(v or '1') for v in self.escape[2:-1].split(';')] if re.fullmatch(r'[\d;]*', self.escape[2:-1]) else []
                    if command in 'Hf':
                        self.y, self.x = (values[0] - 1 if values else 0), (values[1] - 1 if len(values) > 1 else 0)
                    elif command == 'J' and values == [2]:
                        self.cells = [[' '] * self.width for _ in range(self.height)]
                    elif command == 'K' and 0 <= self.y < self.height:
                        self.cells[self.y][max(0, self.x):] = [' '] * (self.width - max(0, self.x))
                elif len(self.escape) == 2 and char in '()':
                    continue
                self.escape = ''
            elif char == '\x1b':
                self.escape = char
            elif char == '\r':
                self.x = 0
            elif char == '\n':
                self.y += 1
            elif char >= ' ':
                if 0 <= self.y < self.height and 0 <= self.x < self.width:
                    self.cells[self.y][self.x] = char
                self.x += 1

    def visible(self):
        return '\n'.join(''.join(row) for row in self.cells)

    def send(self, text):
        self.buffer = b''
        os.write(self.master, text.encode())

    def expect(self, text, timeout=15):
        deadline = time.monotonic() + timeout
        while text not in self.visible():
            if time.monotonic() >= deadline:
                raise AssertionError(f'TUI did not display {text!r}:\n{self.visible()}')
            if self.process.poll() is not None:
                raise AssertionError(f'TUI exited early: {self.process.returncode}')
            if select.select([self.master], [], [], 0.1)[0]:
                try:
                    chunk = os.read(self.master, 65536)
                except OSError as error:
                    if error.errno == errno.EIO:
                        raise AssertionError('TUI closed its terminal') from error
                    raise
                self.buffer += chunk
                self.feed(chunk)
                if b'\x1b[6n' in chunk:
                    os.write(self.master, b'\x1b[1;1R')

    def close(self, control_c=False):
        if self.process.poll() is None:
            self.send('\x03' if control_c else 'q')
            self.process.wait(timeout=15)
        assert self.process.returncode == 0, self.process.returncode
        assert termios.tcgetattr(self.slave) == self.initial, 'Terminal flags were not restored'
        os.close(self.master)
        os.close(self.slave)


def flow(binary, directory, remote=None):
    graph = directory / 'graph.json'
    definition = {'version': 1, 'id': 'pty', 'nodes': [
        {'id': 'input', 'type': 'input', 'name': 'payload'},
        {'id': 'encode', 'type': 'operation', 'operation': 'to_base64', 'args': {}},
        {'id': 'output', 'type': 'output', 'name': 'encoded'}], 'edges': [
        {'id': 'a', 'source_node': 'input', 'source_port': 'data', 'target_node': 'encode', 'target_port': 'data'},
        {'id': 'b', 'source_node': 'encode', 'source_port': 'data', 'target_node': 'output', 'target_port': 'data'}]}
    graph.write_text(json.dumps(definition))
    payload = bytes(range(256))
    (directory / 'input.bin').write_bytes(payload)
    terminal = Terminal(binary, graph, directory, remote)
    try:
        terminal.expect('Graph valid')
        terminal.send('hzz\r'); terminal.expect('HEX requires complete byte pairs')
        terminal.send('f' + str(directory / 'input.bin') + '\r'); terminal.expect('Input bound')
        terminal.resize(20, 8); time.sleep(0.1); terminal.resize(100, 30)
        terminal.send('r'); terminal.expect('Completed run 1')
        terminal.send('e' + str(directory / 'output.bin') + '\r'); terminal.expect('Output exported byte-for-byte')
        assert (directory / 'output.bin').read_bytes() == base64.b64encode(payload)
        terminal.send('s' + str(directory / 'saved.json') + '\r'); terminal.expect('Graph saved without input payloads')
        saved = json.loads((directory / 'saved.json').read_text())
        assert saved['id'] == definition['id'] and 'inputs_base64' not in saved
        terminal.send('l' + str(directory) + '\r'); terminal.expect('Select a graph with arrows')
        terminal.send('\r'); terminal.expect('Graph valid · revision 3')
        terminal.send('l' + str(directory / 'saved.json') + '\r'); terminal.expect('Graph valid · revision 4')
        terminal.send('r'); terminal.expect('Missing input: payload')
        terminal.send('\t'); terminal.expect('Welcome!') if remote is None else terminal.expect('Remote configured')
        terminal.send('\t'); terminal.expect('Missing input: payload')
        # A CPU-bound registered recipe proves worker cancellation in the actual TUI binary.
        definition['nodes'][1] = {'id': 'encode', 'type': 'recipe', 'steps': [
            {'op': 'Derive PBKDF2 key', 'args': ['password', '128', '4000000000', 'SHA256', 'salt']}]}
        graph.write_text(json.dumps(definition))
        terminal.send('l' + str(graph) + '\r'); terminal.expect('Graph valid · revision 5')
        terminal.send('i\r'); terminal.expect('Input bound')
        terminal.send('r'); terminal.expect('Running')
        terminal.send('c'); terminal.expect('cancelled')
        terminal.close(control_c=True)
    finally:
        if terminal.process.poll() is None:
            terminal.process.terminate(); terminal.process.wait(timeout=15)
    # Restart after raw-mode restoration, load the saved definition and quit normally.
    restarted = Terminal(binary, directory / 'saved.json', directory, remote)
    restarted.expect('Graph valid'); restarted.close()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('binary', type=Path)
    parser.add_argument('--server-binary', type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    with tempfile.TemporaryDirectory(prefix='rxchef-tui-pty-') as temporary:
        directory = Path(temporary)
        flow(binary, directory)
        print('PASS local graph: load, bindings, exact export, save/reload, cancellation, resize, Ctrl+C, restart')
        if args.server_binary:
            with socket.socket() as probe:
                probe.bind(('127.0.0.1', 0)); port = probe.getsockname()[1]
            url = f'http://127.0.0.1:{port}'
            server = subprocess.Popen([str(args.server_binary.resolve()), '--listen', f'127.0.0.1:{port}'],
                                      cwd=directory, env={**os.environ, 'RXCHEF_TOKEN': 'pty-test-token-0123456789'},
                                      stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            try:
                deadline = time.monotonic() + 10
                while True:
                    try:
                        with urllib.request.urlopen(url + '/api/v1/health', timeout=1): break
                    except OSError:
                        if time.monotonic() > deadline: raise AssertionError('Server failed to start')
                        time.sleep(0.1)
                flow(binary, directory, url)
                print('PASS remote graph: same byte-exact workflow against real supervised server')
                disconnected = Terminal(binary, directory / 'graph.json', directory, url)
                disconnected.expect('Graph valid'); disconnected.send('i\r'); disconnected.expect('Input bound')
                disconnected.send('r'); disconnected.expect('Running')
                server.terminate(); server.wait(timeout=15)
                disconnected.expect('server request failed', timeout=25)
                disconnected.close()
                print('PASS remote disconnect: error displayed, terminal remains usable and restores flags')
            finally:
                server.terminate(); server.wait(timeout=15)


if __name__ == '__main__':
    main()
