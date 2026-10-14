import type { Graph } from '../../types';

/** Layout only: stable DAG layers, including isolated nodes and unfinished graphs. */
export function arrangeGraph(graph: Graph, sizes: Record<string, { width: number; height: number }> = {}): NonNullable<Graph['presentation']> {
  const ids = new Set(graph.nodes.map(n => n.id));
  const incoming = new Map(graph.nodes.map(n => [n.id, 0]));
  const next = new Map(graph.nodes.map(n => [n.id, [] as string[]]));
  for (const edge of graph.edges) {
    if (!ids.has(edge.source_node) || !ids.has(edge.target_node)) continue;
    incoming.set(edge.target_node, incoming.get(edge.target_node)! + 1);
    next.get(edge.source_node)!.push(edge.target_node);
  }
  const queue = graph.nodes.filter(n => incoming.get(n.id) === 0).map(n => n.id);
  const layers = new Map(queue.map(id => [id, 0]));
  for (let i = 0; i < queue.length; i++) {
    const id = queue[i];
    for (const target of next.get(id)!) {
      layers.set(target, Math.max(layers.get(target) ?? 0, layers.get(id)! + 1));
      incoming.set(target, incoming.get(target)! - 1);
      if (incoming.get(target) === 0) queue.push(target);
    }
  }
  // Cyclic editor drafts remain visible; Rust rejects them before execution.
  const visited = new Set(queue);
  const fallback = Math.max(0, ...layers.values()) + 1;
  const widths = new Map<number, number>();
  for (const node of graph.nodes) {
    if (!visited.has(node.id)) layers.set(node.id, fallback);
    const layer = layers.get(node.id)!;
    widths.set(layer, Math.max(widths.get(layer) ?? 220, sizes[node.id]?.width ?? 220));
  }
  const xs = new Map<number, number>(); let x = 40;
  for (const layer of [...widths.keys()].sort((a, b) => a - b)) { xs.set(layer, x); x += widths.get(layer)! + 100; }
  const ys = new Map<number, number>();
  return Object.fromEntries(graph.nodes.map(node => {
    const layer = layers.get(node.id)!; const y = ys.get(layer) ?? 40;
    ys.set(layer, y + Math.max(140, sizes[node.id]?.height ?? 140) + 60);
    return [node.id, { x: xs.get(layer)!, y }];
  }));
}
