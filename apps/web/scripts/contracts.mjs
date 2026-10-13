import { readFile, writeFile } from 'node:fs/promises';
import { compile } from 'json-schema-to-typescript';
const root = new URL('../src/generated/', import.meta.url);
const schema = JSON.parse(await readFile(new URL('contracts.schema.json', root)));
const text = await compile(schema, 'Contracts', { bannerComment: '/* Generated from Rust serialization. Run npm run contracts. Do not edit. */', additionalProperties: false });
const output = new URL('contracts.ts', root);
if (process.argv.includes('--check')) {
  if (await readFile(output, 'utf8') !== text) throw new Error('TypeScript contracts are stale');
} else await writeFile(output, text);
