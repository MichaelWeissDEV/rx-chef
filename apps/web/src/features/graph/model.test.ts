import { describe, expect, it } from 'vitest';
import { canConnect, ports, starterGraph, executionKey, insertOperation, nextNodeName, graphIssues } from './model';
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


describe('guided graph editing', () => {
  it('inserts consecutive operations into the selected chain without manual wiring', () => {
    const initial = starterGraph();
    const encoded = insertOperation(initial, 'to_base64', 'encode', 'input');
    const roundtrip = insertOperation(encoded, 'from_base64', 'decode', 'encode');
    expect(roundtrip.edges.map(edge => [edge.source_node, edge.target_node])).toEqual([['decode', 'output'], ['input', 'encode'], ['encode', 'decode']]);
    expect(initial.nodes).toHaveLength(2); expect(initial.edges[0].source_node).toBe('input');
    expect(roundtrip.presentation!.output.x).toBeGreaterThan(roundtrip.presentation!.decode.x);
  });
  it('inserts before a selected output while preserving an unrelated fan-out branch', () => {
    const graph = starterGraph(); graph.nodes.push({ id: 'second', type: 'output', name: 'second' });
    graph.edges.push({ ...graph.edges[0], id: 'branch', target_node: 'second' });
    const result = insertOperation(graph, 'to_base64', 'encode', 'output');
    expect(result.edges.find(edge => edge.id === 'branch')!.source_node).toBe('input');
    expect(result.edges.find(edge => edge.id === 'initial-edge')!.source_node).toBe('encode');
  });
  it('keeps argument bindings intact and supports deliberately detached nodes', () => {
    const graph = starterGraph(); graph.nodes.push({ id: 'aes', type: 'operation', operation: 'aes_decrypt' });
    graph.edges.push({ id: 'key', source_node: 'input', source_port: 'data', target_node: 'aes', target_port: 'arg:key' });
    expect(insertOperation(graph, 'to_base64', 'encode', 'input').edges.find(edge => edge.id === 'key')!.source_node).toBe('input');
    expect(insertOperation(graph, 'to_base64', 'encode', 'input', false).edges).toEqual(graph.edges);
  });
  it('distinguishes layout/labels from executable definition edits', () => {
    const graph = starterGraph(); const moved = structuredClone(graph);
    moved.presentation = { input: { x: 999, y: -20 } }; moved.nodes[0].label = 'Readable input';
    expect(executionKey(moved)).toBe(executionKey(graph));
    moved.nodes[0] = { id: 'input', type: 'input', name: 'changed' };
    expect(executionKey(moved)).not.toBe(executionKey(graph));
  });
  it('does not reuse an existing input name after a deletion', () => {
    const graph = starterGraph(); graph.nodes.push({ id: 'extra', type: 'input', name: 'input-2' });
    expect(nextNodeName(graph, 'input')).toBe('input-1');
    graph.nodes.push({ id: 'another', type: 'input', name: 'input-1' });
    expect(nextNodeName(graph, 'input')).toBe('input-3');
  });
  it('guides the user to missing bindings and disconnected outputs', () => {
    const graph = starterGraph(); expect(graphIssues(graph, new Set())).toEqual([{ node: 'input', message: 'Bind input input' }]);
    expect(graphIssues(graph, new Set(['input']))).toEqual([]);
    graph.edges = []; expect(graphIssues(graph, new Set(['input']))).toEqual([{ node: 'output', message: 'Connect output output' }]);
  });
});
