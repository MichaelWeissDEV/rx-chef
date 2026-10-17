import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';
import { resolve,join } from 'node:path';
const root=fileURLToPath(new URL('../../../',import.meta.url));
const target=process.env.CARGO_TARGET_DIR?resolve(root,process.env.CARGO_TARGET_DIR):join(root,'target');
const executable=process.env.RXCHEF_E2E_SERVER_BINARY ? resolve(root,process.env.RXCHEF_E2E_SERVER_BINARY) : join(target,'debug',`rxchef-server${process.platform==='win32'?'.exe':''}`);
console.error(`E2E server binary: ${executable}`);
const child = spawn(executable, ['--listen', '127.0.0.1:18787'], {
  cwd: tmpdir(), env: { ...process.env, RXCHEF_TOKEN: 'browser-test-token-0123456789' }, stdio: 'inherit',
});
for (const signal of ['SIGTERM', 'SIGINT']) process.on(signal, () => child.kill(signal));
child.on('exit', code => process.exit(code ?? 0));
child.on('error',error=>{console.error(error.message);process.exit(1);});
