import { expect, it } from 'vitest';
import { parseGraphResponse } from '../../lib/contracts';
import fixture from '../../generated/contracts.fixture.json';
it('rejects incomplete graph output reported as successful', () => {
  const result = structuredClone(fixture.graph_response);
  result.outputs[0].output_bytes++;
  expect(() => parseGraphResponse(result)).toThrow();
});
it('accepts complete arbitrary binary graph results', () => {
  expect(parseGraphResponse(fixture.graph_response).outputs[0].output_base64).toBe('AP/A');
});
