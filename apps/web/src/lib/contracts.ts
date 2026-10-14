import { parseBase64 } from './bytes';
import * as validators from '../generated/validators.js';
import type { Contracts } from '../generated/contracts';
import { PROTOCOL_VERSION } from '../generated/contracts';
export function parseCapabilities(data:unknown) {
  const capabilities=parseContract('capabilities',data);
  if(capabilities.protocol_version!==PROTOCOL_VERSION) throw {kind:'protocol',message:`Incompatible engine protocol ${capabilities.protocol_version}; expected ${PROTOCOL_VERSION}.`};
  return capabilities;
}
export function parseContract<K extends keyof Contracts>(name: K, data: unknown): Contracts[K] {
  if (!validators[name](data)) throw { kind: 'protocol', message: `Server ${name} response does not match the Rust contract.`, step: null };
  return data as Contracts[K];
}
export function errorMessage(error: unknown): string {
  return error && typeof error === 'object' && 'message' in error ? String(error.message) : 'Request failed';
}

/** A successful graph output must carry its complete authoritative bytes. */
export function parseGraphResponse(data: unknown): Contracts['graph_response'] {
  const result = parseContract('graph_response', data);
  for (const output of result.outputs) {
    const invalid = output.error
      ? output.output_base64 != null || output.output_bytes !== 0
      : output.output_base64 == null || parseBase64(output.output_base64).length !== output.output_bytes;
    if (invalid) throw { kind: 'protocol', message: 'Graph output payload does not match its status or byte count.' };
  }
  return result;
}
