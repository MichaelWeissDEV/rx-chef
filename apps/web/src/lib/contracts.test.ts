import { expect, it } from 'vitest';
import fixtures from '../generated/contracts.fixture.json';
import { parseContract,parseCapabilities } from './contracts';
import type { Contracts } from '../generated/contracts';
it('rejects incompatible engine protocols before sending work',()=>{
  expect(()=>parseCapabilities({...fixtures.capabilities,protocol_version:fixtures.capabilities.protocol_version-1})).toThrow();
});
it('validates real Rust-serialized contracts', () => {
  for (const key of Object.keys(fixtures) as (keyof Contracts)[]) expect(() => parseContract(key, fixtures[key])).not.toThrow();
});
it('covers all 478 registered operations and protocol aliases', () => {
  const ops = parseContract('operations', fixtures.operations);
  expect(ops).toHaveLength(479);
  expect(ops.some(op => op.args.length === 0)).toBe(true);
  for (const kind of ['string', 'boolean', 'integer', 'enum']) expect(ops.some(op => op.args.some(arg => arg.kind === kind))).toBe(true);
  expect(ops.some(op => op.args.some(arg => arg.sensitive))).toBe(true);
  expect(ops.some(op => op.availability !== 'available')).toBe(true);
  expect(ops.some(op => op.aliases.length === 2)).toBe(true);
});
it('rejects renamed wire fields instead of silently losing defaults/categories', () => {
  const wrong = structuredClone(fixtures.operations[0]) as Record<string, unknown>;
  wrong.category = wrong.module; delete wrong.module;
  expect(() => parseContract('operations', [wrong])).toThrow();
  const arg = { ...fixtures.argument } as Record<string, unknown>;
  arg.default_value = arg.default; delete arg.default;
  expect(() => parseContract('argument', arg)).toThrow();
});
it('preserves binary results and error step positions', () => {
  expect(parseContract('result', fixtures.result).output_is_utf8).toBe(false);
  expect(parseContract('error', fixtures.error).step).toBe(2);
});
