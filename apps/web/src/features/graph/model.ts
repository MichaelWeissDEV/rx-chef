import type { Graph, Node, OperationDescriptor } from '../../types';
export type Port = { id: string; type: string };
const byteKinds = new Set(['bytes', 'hex_bytes', 'base64_bytes']);
export function ports(node: Node, operations: OperationDescriptor[]): Port[] {
  switch (node.type) {
    case 'input': return [];
    case 'operation': return [{ id: 'data', type: 'bytes' }, ...(operations.find(op => op.id === node.operation)?.args ?? []).map(arg => ({ id: arg.graph_port_id, type: arg.graph_port_type.type === 'bytes' ? 'bytes' : `arg:${arg.graph_port_type.kind}` }))];
    case 'combine': return 'inputs' in node.combine ? Array.from({ length: Math.min(node.combine.inputs, 64) }, (_, i) => ({ id: `input:${i}`, type: 'bytes' })) : [{ id: 'left', type: 'bytes' }, { id: 'right', type: 'bytes' }];
    default: return [{ id: 'data', type: 'bytes' }];
  }
}
/** Presentation hints only; Rust remains the complete validator and executor. */
export function canConnect(graph: Graph, source: string, target: string, port: string, operations: OperationDescriptor[]): boolean {
  const from = graph.nodes.find(n => n.id === source), to = graph.nodes.find(n => n.id === target);
  if (!from || !to || from.type === 'output' || source === target) return false;
  const type = from.type === 'argument' && !byteKinds.has(from.kind) ? `arg:${from.kind}` : 'bytes';
  if (!ports(to, operations).some(p => p.id === port && p.type === type)) return false;
  if (graph.edges.some(e => e.target_node === target && e.target_port === port)) return false;
  if (to.type === 'operation' && port.startsWith('arg:') && (operations.find(op => op.id === to.operation)?.args ?? []).some(arg => arg.graph_port_id === port && Object.hasOwn(to.args ?? {}, arg.name))) return false;
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
/** UI binding hints only; Rust validates the definition and owns the execution plan. */
export function requiredInputIds(graph:Graph, outputs:string[]=[]):Set<string> {
  const pending=outputs.length?[...outputs]:graph.nodes.filter(node=>node.type==='output').map(node=>node.id);
  const seen=new Set<string>();
  while(pending.length){const id=pending.pop()!;if(seen.has(id))continue;seen.add(id);pending.push(...graph.edges.filter(edge=>edge.target_node===id).map(edge=>edge.source_node));}
  return new Set(graph.nodes.filter(node=>node.type==='input'&&seen.has(node.id)).map(node=>node.id));
}

/** Presentation changes must not invalidate binary results or cancel a worker. */
export function executionKey(graph: Graph): string {
  return JSON.stringify({ ...graph, presentation: undefined, nodes: graph.nodes.map(node => ({ ...node, label: undefined })) });
}
/** Insert a common operation into the selected byte chain without losing branches. */
export function insertOperation(graph: Graph, operation: string, id: string, selected: string, automatic = true): Graph {
  const next = structuredClone(graph);
  const selectedNode = next.nodes.find(node => node.id === selected);
  const outputEdge = selectedNode?.type === 'output' ? next.edges.find(edge => edge.target_node === selected && edge.target_port === 'data') : undefined;
  const source = outputEdge?.source_node ?? (selectedNode && selectedNode.type !== 'output' && selectedNode.type !== 'argument' ? selected : undefined);
  const outgoing = automatic && source ? next.edges.filter(edge => edge.source_node === source && edge.target_port === 'data' && (!outputEdge || edge.id === outputEdge.id)) : [];
  const position = next.presentation?.[source ?? selected] ?? { x: 40, y: 80 };
  next.nodes.push({ id, type: 'operation', operation, args: {}, enabled: true });
  next.presentation = { ...next.presentation, [id]: { x: position.x + 280, y: position.y } };
  if (automatic && source) {
    next.edges = next.edges.map(edge => outgoing.some(out => out.id === edge.id) ? { ...edge, source_node: id } : edge);
    next.edges.push({ id: `edge-${id}`, source_node: source, source_port: 'data', target_node: id, target_port: 'data' });
    // Make room in the downstream chain; unrelated branches retain their layout.
    const pending = outgoing.map(edge => edge.target_node); const moved = new Set<string>();
    while (pending.length) {
      const target = pending.pop()!; if (target === id || moved.has(target)) continue; moved.add(target);
      const old = next.presentation[target];
      if (old) next.presentation[target] = { ...old, x: old.x + 280 };
      pending.push(...next.edges.filter(edge => edge.source_node === target).map(edge => edge.target_node));
    }
  }
  return next;
}
export function graphIssues(graph: Graph, bound: Set<string>, outputs: string[] = []): { node: string; message: string }[] {
  const required = requiredInputIds(graph, outputs);
  const issues = graph.nodes.filter(node => required.has(node.id) && !bound.has(node.id))
    .map(node => ({ node: node.id, message: `Bind input ${'name' in node ? node.name : node.id}` }));
  for (const node of graph.nodes.filter(node => node.type === 'output' && (!outputs.length || outputs.includes(node.id)))) {
    if (!graph.edges.some(edge => edge.target_node === node.id)) issues.push({ node: node.id, message: `Connect output ${'name' in node ? node.name : node.id}` });
  }
  if (!graph.nodes.some(node => node.type === 'output')) issues.push({ node: '', message: 'Add an output to see a result' });
  return issues;
}

export function nextNodeName(graph: Graph, type: 'input' | 'output'): string {
  const names = new Set(graph.nodes.filter(node => node.type === type).map(node => 'name' in node ? node.name : ''));
  let index = 1; while (names.has(`${type}-${index}`)) index++;
  return `${type}-${index}`;
}
