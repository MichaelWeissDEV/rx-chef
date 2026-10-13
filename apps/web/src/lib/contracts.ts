import Ajv2020 from 'ajv/dist/2020';
import schema from '../generated/contracts.schema.json';
import type { Contracts } from '../generated/contracts';
const ajv = new Ajv2020({ strict: false, allErrors: true });
ajv.addSchema(schema, 'contracts');
const validators = new Map<string, ReturnType<typeof ajv.compile>>();
export function parseContract<K extends keyof Contracts>(name: K, data: unknown): Contracts[K] {
  let validate = validators.get(name);
  if (!validate) { validate = ajv.compile({ $ref: `contracts#/properties/${name}` }); validators.set(name, validate); }
  if (!validate(data)) throw { kind: 'protocol', message: `Server ${name} response does not match the Rust contract.`, step: null };
  return data as Contracts[K];
}
export function errorMessage(error: unknown): string {
  return error && typeof error === 'object' && 'message' in error ? String(error.message) : 'Request failed';
}
