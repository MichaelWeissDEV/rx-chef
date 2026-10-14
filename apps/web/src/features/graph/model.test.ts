import { describe, expect, it } from 'vitest';
import { canConnect, ports, starterGraph } from './model';
import type { OperationDescriptor } from '../../types';
import fixture from '../../generated/contracts.fixture.json';
const operations = fixture.operations as OperationDescriptor[];
describe('graph presentation contracts', () => {
  it('rejects duplicate single-port connections and output sources', () => {
    const graph = starterGraph();
    expect(canConnect(graph, 'input', 'output', 'data', operations)).toBe(false);
    expect(canConnect(graph, 'output', 'input', 'data', operations)).toBe(false);
  });
  it('derives argument ports from registry metadata', () => {
    expect(ports({ id: 'aes', type: 'operation', operation: 'aes_decrypt' }, operations)).toContainEqual({ id: 'arg:key', type: 'bytes' });
  });
  it('allows real multi-input fan-in but rejects cycles', () => {
    const graph = starterGraph();
    graph.nodes.push({ id: 'concat', type: 'combine', combine: { kind: 'concatenate', inputs: 2 } });
    expect(canConnect(graph, 'input', 'concat', 'input:0', operations)).toBe(true);
    graph.edges.push({ id: 'cycle', source_node: 'concat', source_port: 'data', target_node: 'concat', target_port: 'input:1' });
    expect(canConnect(graph, 'concat', 'concat', 'input:0', operations)).toBe(false);
  });
  it('requires an explicit adapter for non-byte argument ports', () => {
    const graph = starterGraph();
    graph.nodes.push({ id: 'hash', type: 'operation', operation: 'sha2' });
    const port = ports(graph.nodes[2], operations).find(p => p.id.startsWith('arg:'))!;
    expect(canConnect(graph, 'input', 'hash', port.id, operations)).toBe(false);
  });
});

it('uses Rust-provided port IDs and types for all registered descriptors', () => {
  for (const op of operations) {
    const actual = ports({ id: 'operation', type: 'operation', operation: op.id }, operations).slice(1);
    expect(actual).toEqual(op.args.map(arg => ({ id: arg.graph_port_id, type: arg.graph_port_type.type === 'bytes' ? 'bytes' : `arg:${arg.graph_port_type.kind}` })));
    expect(new Set(actual.map(port => port.id)).size).toBe(actual.length);
  }
});
