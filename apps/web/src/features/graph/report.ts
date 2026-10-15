import type { Graph, GraphResult, Node, OperationDescriptor } from '../../types';
import { arrangeGraph } from './layout';
import { ports } from './model';
export type EdgeStyle = 'smoothstep' | 'step' | 'default';
export interface TransformationStep { id: string; title: string; description: string; details: string[]; status: string }
const plain = (text: string) => text.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
/** Dependency order is explanatory; independent branches may execute in a different order. */
export function transformationSteps(graph: Graph, operations: OperationDescriptor[], result: GraphResult | null): TransformationStep[] {
  const pending = [...graph.nodes]; const ordered: Node[] = []; const seen = new Set<string>();
  while (pending.length) {
    const index = pending.findIndex(node => graph.edges.filter(edge => edge.target_node === node.id).every(edge => seen.has(edge.source_node)));
    const node = pending.splice(index < 0 ? 0 : index, 1)[0]; ordered.push(node); seen.add(node.id);
  }
  const title = (node: Node) => node.label || ('name' in node ? node.name : node.type === 'operation' ? operations.find(op => op.id === node.operation)?.name ?? node.operation : node.type);
  const operationDetails = (id: string, args: Record<string, string>, incoming: Graph['edges']) => {
    const descriptor = operations.find(op => op.id === id || op.name === id);
    const keys = new Set([...(descriptor?.args.map(arg => arg.name) ?? []), ...Object.keys(args)]);
    return [...keys].map(name => {
      const arg = descriptor?.args.find(arg => arg.name === name); const binding = incoming.find(edge => edge.target_port === arg?.graph_port_id);
      const value = binding ? `bound from ${binding.source_node}` : !arg || arg.sensitive ? '[redacted]' : Object.hasOwn(args, name) ? args[name] : `${arg.default} (default)`;
      return `${name}: ${value}`;
    });
  };
  return ordered.map(node => {
    const incoming = graph.edges.filter(edge => edge.target_node === node.id);
    const details = incoming.map(edge => `${edge.source_node}.${edge.source_port} → ${edge.target_port}`);
    const op = node.type === 'operation' ? operations.find(op => op.id === node.operation) : undefined;
    let description = node.type === 'input' ? 'Reads explicitly bound bytes; encoding is selected in the input inspector.' : node.type === 'output' ? 'Exposes the complete bytes arriving at its data port.' : node.type === 'argument' ? `Adapts incoming bytes to ${node.kind}.` : node.type === 'combine' ? `Combines connected inputs using ${node.combine.kind}.` : node.type === 'recipe' ? 'Runs the embedded recipe in the declared step order.' : plain(op?.description ?? 'Operation descriptor is unavailable.');
    if (node.type === 'operation') { details.push(...operationDetails(node.operation, node.args ?? {}, incoming)); if (node.enabled === false) description = 'Disabled: passes data through without applying the operation. ' + description; }
    if (node.type === 'combine') details.push(...Object.entries(node.combine).filter(([key]) => key !== 'kind').map(([key, value]) => `${key}: ${value}`));
    if (node.type === 'recipe') node.steps.forEach((step, index) => {
      const descriptor = operations.find(op => op.id === step.op || op.name === step.op);
      const args = Object.fromEntries((step.args ?? []).map((value, i) => [descriptor?.args[i]?.name ?? `argument ${i + 1}`, value]));
      details.push(`Step ${index + 1}: ${step.op}`, plain(descriptor?.description ?? 'Descriptor unavailable.'), ...operationDetails(step.op, args, []));
    });
    const run = result?.nodes.find(item => item.node_id === node.id);
    return { id: node.id, title: `${title(node)} · ${node.type}`, description, details, status: run ? `${run.status} · ${run.output_bytes} output bytes · ${run.elapsed_ms.toFixed(2)} ms` : 'Not executed in the current revision' };
  });
}
const escape = (text: string) => Array.from(text).filter(char => { const cp = char.codePointAt(0)!; return cp === 9 || cp === 10 || cp === 13 || cp >= 32 && cp <= 0xd7ff || cp >= 0xe000 && cp <= 0xfffd || cp >= 0x10000 && cp <= 0x10ffff; }).join('').replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&apos;' })[char]!);
const wrap = (text: string, width = 100) => {
  const chars = Array.from(text).filter(char => char.codePointAt(0)! >= 32 || char === '\n' || char === '\t' || char === '\r');
  const clipped = chars.length > 1000 ? [...chars.slice(0, 997), '…'] : chars;
  const lines: string[] = []; for (let i = 0; i < clipped.length; i += width) lines.push(clipped.slice(i, i + width).join(''));
  return lines.length ? lines : [''];
};
/** Standalone SVG: escaped text only, no scripts, external resources, payloads or foreignObject. */
export function graphSvg(graph: Graph, operations: OperationDescriptor[], result: GraphResult | null, style: EdgeStyle): string {
  const layout = arrangeGraph(graph); const positions = Object.fromEntries(graph.nodes.map(node => {
    const p = graph.presentation?.[node.id] ?? layout[node.id];
    return [node.id, { x: Number.isFinite(p.x) ? Math.max(-100000, Math.min(100000, p.x)) : 0, y: Number.isFinite(p.y) ? Math.max(-100000, Math.min(100000, p.y)) : 0 }];
  }));
  const minX = Math.min(0, ...Object.values(positions).map(p => p.x)); const minY = Math.min(0, ...Object.values(positions).map(p => p.y));
  for (const p of Object.values(positions)) { p.x = p.x - minX + 30; p.y = p.y - minY + 80; }
  const width = Math.max(1000, ...Object.values(positions).map(p => p.x + 290));
  const nodeHeight = (node: Node) => 70 + ports(node, operations).length * 20;
  const diagramBottom = Math.max(150, ...graph.nodes.map(node => positions[node.id].y + nodeHeight(node))) + 40;
  const steps = transformationSteps(graph, operations, result);
  const lines = steps.flatMap((step, i) => [`${i + 1}. ${step.title} [${step.id}]`, ...wrap(step.description), ...step.details.flatMap(detail => wrap(detail)), step.status, '']);
  const text = (x: number, y: number, value: string, size = 13) => `<text x="${x}" y="${y}" font-size="${size}">${escape(value)}</text>`;
  const edges = graph.edges.map(edge => {
    const from = graph.nodes.find(node => node.id === edge.source_node), to = graph.nodes.find(node => node.id === edge.target_node); if (!from || !to) return '';
    const source = positions[from.id], target = positions[to.id]; const x1 = source.x + 250, y1 = source.y + nodeHeight(from) / 2, x2 = target.x, y2 = target.y + 72 + Math.max(0, ports(to, operations).findIndex(port => port.id === edge.target_port)) * 20;
    const middle = (x1 + x2) / 2;
    const r = Math.min(12, Math.abs(x2 - x1) / 4, Math.abs(y2 - y1) / 2); const dx = x2 >= x1 ? 1 : -1, dy = y2 >= y1 ? 1 : -1;
    const path = style === 'default' ? `M${x1},${y1} C${middle},${y1} ${middle},${y2} ${x2},${y2}` : style === 'step' ? `M${x1},${y1} H${middle} V${y2} H${x2}` : `M${x1},${y1} H${middle - dx * r} Q${middle},${y1} ${middle},${y1 + dy * r} V${y2 - dy * r} Q${middle},${y2} ${middle + dx * r},${y2} H${x2}`;
    return `<path d="${path}" fill="none" stroke="#64748b" stroke-width="2" marker-end="url(#arrow)"/>`;
  }).join('');
  const cards = graph.nodes.map(node => { const p = positions[node.id], step = steps.find(step => step.id === node.id)!; return `<rect x="${p.x}" y="${p.y}" width="250" height="${nodeHeight(node)}" rx="8" fill="#f1f5f9" stroke="#64748b"/>` + text(p.x + 12, p.y + 24, Array.from(step.title).slice(0, 32).join('')) + text(p.x + 12, p.y + 46, Array.from(node.id).slice(0, 36).join(''), 10) + ports(node, operations).map((port, i) => text(p.x + 12, p.y + 76 + i * 20, port.id, 10)).join(''); }).join('');
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${diagramBottom + 100 + lines.length * 19}" viewBox="0 0 ${width} ${diagramBottom + 100 + lines.length * 19}"><title>${escape(graph.id)} — data flow and transformation steps</title><desc>Definition and available runtime metadata. No input/output payloads. Sensitive arguments redacted. Long fields limited to 1000 characters. Independent branches are listed in dependency order, not a timing trace.</desc><defs><marker id="arrow" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 Z" fill="#64748b"/></marker></defs><rect width="100%" height="100%" fill="white"/><g fill="#0f172a" font-family="sans-serif">${text(30, 32, graph.id, 20)}${edges}${cards}${text(30, diagramBottom, 'Transformation steps · definition and available run metadata', 18)}${text(30, diagramBottom + 24, 'No input/output payloads. Sensitive arguments redacted. Fields limited to 1000 characters.', 12)}${lines.map((line, i) => text(30, diagramBottom + 56 + i * 19, line)).join('')}</g></svg>\n`;
}
