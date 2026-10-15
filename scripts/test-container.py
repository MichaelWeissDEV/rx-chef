#!/usr/bin/env python3
"""Exercise the real runtime image without host ports or external networking."""
import argparse
import base64
import json
from pathlib import Path
import subprocess
import time
import uuid
import urllib.request
import urllib.error

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--image', default='rxchef-server:local')
parser.add_argument('--variant', choices=['web', 'headless'], default='web')
parser.add_argument('--host-network', action='store_true', help='Also verify host access at 127.0.0.1:8787; requires a free port and Desktop opt-in')
args = parser.parse_args()
name = 'rxchef-smoke-' + uuid.uuid4().hex[:12]
token = 'isolated-container-smoke-0123456789'


def docker(*arguments, payload=None):
    return subprocess.check_output(['docker', *arguments], input=payload, timeout=30)


def request(path, payload=None, authenticated=True):
    command = ['exec', '-i', name, 'curl', '--silent', '--show-error', '--max-time', '10',
               '--write-out', '\n%{http_code}']
    if authenticated:
        command += ['--header', 'Authorization: Bearer ' + token]
    if payload is not None:
        command += ['--header', 'Content-Type: application/json', '--data-binary', '@-']
    command += ['http://127.0.0.1:8787' + path]
    raw = docker(*command, payload=json.dumps(payload).encode() if payload is not None else None)
    body, status = raw.rsplit(b'\n', 1)
    return int(status), body


try:
    docker('run', '-d', '--name', name, '--network', 'host' if args.host_network else 'none', '--read-only',
           '--tmpfs', '/tmp:rw,noexec,nosuid,size=64m', '--cap-drop', 'ALL',
           '--security-opt', 'no-new-privileges:true', '--pids-limit', '256',
           '--memory', '2g', '--health-interval', '1s', '--health-start-period', '1s',
           '-e', 'RXCHEF_TOKEN=' + token, args.image)
    for _ in range(100):
        state = json.loads(docker('inspect', name))[0]
        if not state['State']['Running']:
            raise AssertionError(docker('logs', name).decode())
        if state['State'].get('Health', {}).get('Status') == 'healthy':
            break
        time.sleep(.1)
    else:
        raise AssertionError('image did not become healthy')
    assert state['Config']['User'] == '65532:65532'
    assert state['HostConfig']['ReadonlyRootfs']
    assert request('/api/v1/health', authenticated=False)[0] == 200
    assert request('/api/v1/capabilities', authenticated=False)[0] == 401
    status, body = request('/api/v1/capabilities')
    assert status == 200 and json.loads(body)['protocol_version'] == 2
    status, body = request('/', authenticated=args.variant == 'headless')
    assert status == (200 if args.variant == 'web' else 404)
    if args.variant == 'web':
        assert b'<html' in body.lower()
    graph = json.loads((Path(__file__).resolve().parents[1] / 'examples/graphs/three-inputs.rxgraph.json').read_text())
    inputs = {'payload': bytes(range(256)) + b'\x00\xff\xc0', 'second': b'\x00\xff', 'third': b'last\x00'}
    status, body = request('/api/v1/graphs/execute', {'graph': graph,
        'inputs_base64': {key: base64.b64encode(value).decode() for key, value in inputs.items()}, 'outputs': []})
    assert status == 200, body
    outputs = {item['name']: base64.b64decode(item['output_base64'], validate=True) for item in json.loads(body)['outputs']}
    assert outputs['combined'] == b''.join(inputs.values())
    assert outputs['original'] == inputs['payload']
    assert outputs['encoded'] == base64.b64encode(inputs['payload'])
    job = {'method': 'execute', 'params': {'input_base64': 'AP+A', 'recipe': [], 'variables': {}}}
    status, body = request('/api/v1/jobs', job)
    assert status == 200, body
    identity = json.loads(body)['id']
    for _ in range(100):
        status, body = request('/api/v1/jobs/' + identity)
        assert status == 200, body
        result = json.loads(body)
        if result['status'] not in ['queued', 'running']:
            break
        time.sleep(.05)
    assert result['status'] == 'succeeded', result
    assert base64.b64decode(result['result']['value']['result']['output_base64'], validate=True) == b'\x00\xff\x80'
    if args.host_network:
        probe = urllib.request.Request('http://127.0.0.1:8787/api/v1/capabilities', headers={'Authorization': 'Bearer ' + token})
        try:
            with urllib.request.urlopen(probe, timeout=5) as response:
                assert json.load(response)['protocol_version'] == 2
        except urllib.error.URLError as error:
            raise SystemExit(f'Host loopback is unreachable: {error}. On Docker Desktop enable Host networking; no settings were changed.')
        print('PASS host loopback access with authenticated capabilities')
    docker('stop', '--time', '10', name)
    state = json.loads(docker('inspect', name))[0]
    assert state['State']['ExitCode'] == 0, state['State']
    print(f'PASS {args.variant}: non-root/read-only, health, auth, assets, exact multi-input worker outputs, jobs and SIGTERM shutdown')
finally:
    subprocess.run(['docker', 'rm', '-f', name], capture_output=True, timeout=30)
