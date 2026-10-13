import { cp, mkdir } from 'node:fs/promises';
// Dedicated generated directory, included explicitly by cargo package.
const destination = new URL('../../../crates/server/web-dist/', import.meta.url);
await mkdir(destination, { recursive: true });
await cp(new URL('../dist/', import.meta.url), destination, { recursive: true });
