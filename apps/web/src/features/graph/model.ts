import type { Graph, Node, OperationDescriptor } from '../../types';
export type Port = { id: string; type: string };
export const argumentId = (name: string) => name.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '_').replace(/^_|_$/g, '');
export function ports(node: Node, operations: OperationDescriptor[]): Port[] {
  switch (node.type) {
    case 'input': return [];
    case 'operation': return [{ id: 'data', type: 'bytes' }, ...(operations.find(op => op.id === node.operation)?.args ?? []).map(arg => ({ id: `arg:${argumentId(arg.name)}`, type: ['bytes', 'hex_bytes', 'base64_bytes'].includes(arg.kind) ? 'bytes' : `arg:${arg.kind}` }))];
    case 'combine': return 'inputs' in node.combine ? Array.from({ length: Math.min(node.combine.inputs, 64) }, (_, i) => ({ id: `input:${i}`, type: 'bytes' })) : [{ id: 'left', type: 'bytes' }, { id: 'right', type: 'bytes' }];
    default: return [{ id: 'data', type: 'bytes' }];
  }
}
/** Presentation hints only; Rust remains the complete validator and executor. */
export function canConnect(graph: Graph, source: string, target: string, port: string, operations: OperationDescriptor[]): boolean {
  const from = graph.nodes.find(n => n.id === source), to = graph.nodes.find(n => n.id === target);
  if (!from || !to || from.type === 'output' || source === target) return false;
  const type = from.type === 'argument' ? `arg:${from.kind}` : 'bytes';
  if (!ports(to, operations).some(p => p.id === port && p.type === type)) return false;
  if (graph.edges.some(e => e.target_node === target && e.target_port === port)) return false;
  if (to.type === 'operation' && port.startsWith('arg:') && Object.keys(to.args ?? {}).some(name => `arg:${argumentId(name)}` === port)) return false;
  const queue = [target], seen = new Set<string>();
  while (queue.length) {
    const id = queue.pop()!; if (id === source) return false;
    if (seen.has(id)) continue; seen.add(id);
    queue.push(...graph.edges.filter(e => e.source_node === id).map(e => e.target_node));
  }
  return true;
}
export function starterGraph(): Graph {
  return { version: 1, id: 'untitled', nodes: [{ id: 'input', type: 'input', name: 'input' }, { id: 'output', type: 'output', name: 'output' }], edges: [{ id: 'initial-edge', source_node: 'input', source_port: 'data', target_node: 'output', target_port: 'data' }], presentation: { input: { x: 40, y: 80 }, output: { x: 420, y: 80 } } };
}
