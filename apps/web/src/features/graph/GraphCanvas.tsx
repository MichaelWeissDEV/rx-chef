import { ReactFlow, Background, Controls, MiniMap, Handle, Position, type Connection, type Edge, type Node as FlowNode, type NodeChange, type NodeProps, MarkerType } from '@xyflow/react';
import type { Node } from '../../types';
export type GraphCardNode = FlowNode<{ domain: Node; title: string; ports: string[]; status: string; category: string; summary: string; timing?: string }>;
function GraphCard({ data }: NodeProps<GraphCardNode>) {
  const portTitle = (port: string) => port === 'data' ? 'Data' : port.startsWith('input:') ? `Input ${Number(port.slice(6)) + 1}` : port.replace('arg:', '');
  return <div className={`graph-card graph-card-${data.domain.type}`} data-state={data.status}><div className="graph-card-kind">{data.category}</div><strong>{data.title}</strong><small className="graph-node-status">{data.status === 'Node status pending' ? 'Waiting' : data.status}{data.timing && ` · ${data.timing}`}</small>{data.summary && <small>{data.summary}</small>}
    {data.ports.map(port => <div className="graph-port" key={port} title={port}><Handle type="target" position={Position.Left} id={port} aria-label={`Connect to ${data.title}: ${portTitle(port)}`} style={{ top: '50%', left: -15 }} />{portTitle(port)}</div>)}
    {data.domain.type !== 'output' && <Handle type="source" position={Position.Right} id="data" aria-label={`Connect from ${data.title}`} />}
  </div>;
}
const nodeTypes = { graph: GraphCard };
interface Props { nodes: GraphCardNode[]; edges: Edge[]; theme: 'dark' | 'light'; changes(changes: NodeChange<GraphCardNode>[]): void;
  connect(connection: Connection): void; valid(connection: Connection | Edge): boolean; deleteEdges(edges: Edge[]): void;
  reconnect(edge: Edge, connection: Connection): void; drop(event: React.DragEvent): void;
}
export function GraphCanvas({ nodes, edges, theme, changes, connect, valid, deleteEdges, reconnect, drop }: Props) {
  return <div className="graph-canvas" onDragOver={e => e.preventDefault()} onDrop={drop}><ReactFlow colorMode={theme} nodes={nodes} edges={edges} nodeTypes={nodeTypes} onNodesChange={changes} onConnect={connect} isValidConnection={valid} onEdgesDelete={deleteEdges} onReconnect={reconnect} panOnDrag selectionKeyCode="Shift" defaultEdgeOptions={{ type: 'smoothstep', markerEnd: { type: MarkerType.ArrowClosed } }} fitView><Background /><Controls /><MiniMap /></ReactFlow></div>;
}
