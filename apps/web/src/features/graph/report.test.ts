import { expect, it } from 'vitest';
import fixture from '../../generated/contracts.fixture.json';
import type { OperationDescriptor, Graph } from '../../types';
import { graphSvg, transformationSteps } from './report';
import { starterGraph, insertOperation } from './model';
const operations = fixture.operations as OperationDescriptor[];
it('reports dependencies, defaults and actual run metadata without output payloads', () => {
  const graph = insertOperation(starterGraph(), 'to_base64', 'encode', 'input');
  graph.nodes.reverse();
  const result = { nodes: [{ node_id: 'encode', status: 'succeeded' as const, output_bytes: 4, elapsed_ms: 1.25 }], outputs: [{ node_id: 'output', name: 'output', output_base64: 'U0VDUkVU', output_bytes: 6 }] };
  const steps = transformationSteps(graph, operations, result);
  expect(steps.map(step => step.id)).toEqual(['input', 'encode', 'output']);
  expect(steps[1].details).toContain('input.data → data');
  expect(steps[1].details.some(detail => detail.includes('(default)'))).toBe(true);
  expect(steps[1].status).toContain('4 output bytes');
  expect(graphSvg(graph, operations, result, 'default')).not.toContain('U0VDUkVU');
});
it('redacts sensitive operation and embedded recipe arguments while preserving ordinary ones', () => {
  const graph: Graph = { ...starterGraph(), nodes: [
    { id: 'aes', type: 'operation', operation: 'aes_decrypt', args: { Key: 'PRIVATE-KEY', Input: 'Raw' } },
    { id: 'recipe', type: 'recipe', steps: [{ op: 'AES Decrypt', args: ['PRIVATE-EMBEDDED-KEY'] }] },
  ], edges: [] };
  const text = JSON.stringify(transformationSteps(graph, operations, null));
  expect(text).not.toContain('PRIVATE-KEY'); expect(text).not.toContain('PRIVATE-EMBEDDED-KEY');
  expect(text).toContain('[redacted]'); expect(text).toContain('Raw');
});
it('creates a standalone escaped image for a disconnected draft and bounds invalid positions', () => {
  const graph = starterGraph(); graph.id = '<script>alert("x")</script>';
  graph.nodes[0].label = '<image href="https://example.test/"/>';
  graph.presentation!.input = { x: NaN, y: Infinity };
  const svg = graphSvg(graph, operations, null, 'step');
  expect(svg).toContain('&lt;script&gt;'); expect(svg).not.toContain('<script>'); expect(svg).not.toContain('<image ');
  expect(svg).not.toMatch(/NaN|Infinity|foreignObject/); expect(svg).toContain('Transformation steps');
  expect(svg).toContain('Not executed in the current revision'); expect(svg).toContain('marker-end="url(#arrow)"');
});
it('exports orthogonal and flowing paths distinctly and tolerates cyclic drafts', () => {
  const graph = starterGraph();
  expect(graphSvg(graph, operations, null, 'step')).toMatch(/ H[\d.]+ V[\d.]+ H/);
  expect(graphSvg(graph, operations, null, 'default')).toMatch(/ C/);
  graph.edges.push({ id: 'cycle', source_node: 'output', source_port: 'data', target_node: 'input', target_port: 'data' });
  expect(transformationSteps(graph, operations, null)).toHaveLength(2);
});
