import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';
const child = spawn(fileURLToPath(new URL('../../../target/debug/rxchef-server', import.meta.url)), ['--listen', '127.0.0.1:18787'], {
  cwd: tmpdir(), env: { ...process.env, RXCHEF_TOKEN: 'browser-test-token-0123456789' }, stdio: 'inherit',
});
for (const signal of ['SIGTERM', 'SIGINT']) process.on(signal, () => child.kill(signal));
child.on('exit', code => process.exit(code ?? 0));
