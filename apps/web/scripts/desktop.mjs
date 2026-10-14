import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
const cli = fileURLToPath(new URL('../node_modules/@tauri-apps/cli/tauri.js', import.meta.url));
const child = spawn(process.execPath, [cli, ...process.argv.slice(2)], { cwd: fileURLToPath(new URL('../../desktop/', import.meta.url)), stdio: 'inherit' });
child.on('exit', code => process.exit(code ?? 1));
