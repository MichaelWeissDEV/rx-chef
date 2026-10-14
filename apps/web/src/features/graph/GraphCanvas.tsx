import { ReactFlow, Background, Controls, MiniMap, Handle, Position, type Connection, type Edge, type Node as FlowNode, type NodeChange, type NodeProps } from '@xyflow/react';
import type { Node } from '../../types';
export type GraphCardNode = FlowNode<{ domain: Node; title: string; ports: string[]; status: string; category: string; summary: string; timing?: string }>;
function GraphCard({ data }: NodeProps<GraphCardNode>) {
  return <div className="graph-card"><small>{data.category}</small><strong>{data.title}</strong><small>{data.status}{data.timing && ` · ${data.timing}`}</small>{data.summary && <small>{data.summary}</small>}
    {data.ports.map((port, i) => <div className="graph-port" key={port}><Handle type="target" position={Position.Left} id={port} style={{ top: 83 + i * 24 }} />{port}</div>)}
    {data.domain.type !== 'output' && <Handle type="source" position={Position.Right} id="data" />}
  </div>;
}
const nodeTypes = { graph: GraphCard };
interface Props { nodes: GraphCardNode[]; edges: Edge[]; theme: 'dark' | 'light'; changes(changes: NodeChange<GraphCardNode>[]): void;
  connect(connection: Connection): void; valid(connection: Connection | Edge): boolean; deleteEdges(edges: Edge[]): void;
  reconnect(edge: Edge, connection: Connection): void; drop(event: React.DragEvent): void;
}
export function GraphCanvas({ nodes, edges, theme, changes, connect, valid, deleteEdges, reconnect, drop }: Props) {
  return <div className="graph-canvas" onDragOver={e => e.preventDefault()} onDrop={drop}><ReactFlow colorMode={theme} nodes={nodes} edges={edges} nodeTypes={nodeTypes} onNodesChange={changes} onConnect={connect} isValidConnection={valid} onEdgesDelete={deleteEdges} onReconnect={reconnect} selectionOnDrag fitView><Background /><Controls /><MiniMap /></ReactFlow></div>;
}
