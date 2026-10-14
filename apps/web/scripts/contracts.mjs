import { readFile, writeFile } from 'node:fs/promises';
import { compile } from 'json-schema-to-typescript';
import Ajv2020 from 'ajv/dist/2020.js';
import standalone from 'ajv/dist/standalone/index.js';
const root = new URL('../src/generated/', import.meta.url);
const schema = JSON.parse(await readFile(new URL('contracts.schema.json', root)));
const text = await compile(schema, 'Contracts', { bannerComment: '/* Generated from Rust serialization. Run npm run contracts:generate. Do not edit. */', additionalProperties: false });
const ajv = new Ajv2020({ strict: false, allErrors: true, code: { source: true, esm: true } });
for (const format of ['uint', 'uint32', 'uint64', 'double']) ajv.addFormat(format, true);
ajv.addSchema(schema, 'contracts');
const refs = {};
for (const name of Object.keys(schema.properties)) {
  ajv.addSchema({ $ref: `contracts#/properties/${name}` }, `contract-${name}`);
  refs[name] = `contract-${name}`;
}
const validators = standalone(ajv, refs).replaceAll('require("ajv/dist/runtime/ucs2length").default', 'ucs2length');
const js = '/* Generated validators: no runtime eval. */\nimport ucs2length from "ajv/dist/runtime/ucs2length.js";\n' + validators;
for (const [name, content] of [['contracts.ts', text], ['validators.js', js], ['validators.d.ts', 'import type { Contracts } from \'./contracts\';\n' + Object.keys(schema.properties).map(name => `export const ${name}: (data: unknown) => data is Contracts['${name}'];`).join('\n') + '\n']]) {
  const output = new URL(name, root);
  if (process.argv.includes('--check')) { if (await readFile(output, 'utf8') !== content) throw new Error(`${name} is stale`); }
  else await writeFile(output, content);
}
