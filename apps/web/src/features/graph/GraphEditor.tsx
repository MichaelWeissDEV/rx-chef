import { useEffect, useMemo, useReducer, useRef, useState } from 'react';
import { ReactFlow, ReactFlowProvider, useReactFlow, useNodesInitialized, Background, Controls, MiniMap, Handle, Position, type Connection, type NodeProps, type Node as FlowNode, type NodeChange } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import type { Graph, GraphResult, Node, OperationDescriptor, RxchefClient } from '../../types';
import { parseContract, parseGraphResponse, errorMessage } from '../../lib/contracts';
import { encodeBase64, parseBase64, parseInput, downloadBytes, encodeHex, previewUtf8 } from '../../lib/bytes';
import { historyReducer } from '../../lib/history';
import { ports, canConnect, starterGraph } from './model';
import './graph.css';

type WithoutId<T> = T extends unknown ? Omit<T, 'id'> : never;
type Card = FlowNode<{ domain: Node; title: string; ports: string[]; status: string }>;
function GraphCard({ data }: NodeProps<Card>) {
  return <div className="graph-card"><strong>{data.title}</strong><small>{data.status}</small>
    {data.ports.map((port, i) => <div className="graph-port" key={port}>
      <Handle type="target" position={Position.Left} id={port} style={{ top: 60 + i * 24 }} />{port}
    </div>)}
    {data.domain.type !== 'output' && <Handle type="source" position={Position.Right} id="data" />}
  </div>;
}
const nodeTypes = { graph: GraphCard };
type Props = { client: RxchefClient; operations: OperationDescriptor[]; theme: 'dark' | 'light'; visible: boolean };
export function GraphEditor(props: Props) { return <ReactFlowProvider><Editor {...props} /></ReactFlowProvider>; }
function Editor({ client, operations, theme, visible }: Props) {
  const flow = useReactFlow<Card>();
  const initialized = useNodesInitialized();
  const [fitRevision, setFitRevision] = useState(0);
  const [sizes, setSizes] = useState<Record<string, { width: number; height: number }>>({});
  const [history, dispatch] = useReducer(historyReducer<Graph>, { past: [], present: starterGraph(), future: [] });
  const graph = history.present;
  useEffect(() => {
    if (initialized && visible) requestAnimationFrame(() => { void flow.fitView({ padding: 0.15 }); });
  }, [initialized, graph.nodes.length, fitRevision, flow, visible]);
  const [selected, select] = useState('input');
  const [values, setValues] = useState<Record<string, { value: string; mode: 'text' | 'hex' | 'base64' }>>({});
  const [result, setResult] = useState<GraphResult | null>(null);
  const [output, setOutput] = useState('');
  const [view, setView] = useState<'text' | 'hex' | 'base64'>('hex');
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [running, setRunning] = useState(false);
  const [canCancel, setCanCancel] = useState(false);
  const [source, setSource] = useState('input'); const [target, setTarget] = useState('output'); const [port, setPort] = useState('data');
  const [operation, setOperation] = useState('to_base64');
  const [combine, setCombine] = useState('concatenate');
  const job = useRef<string | null>(null); const cancelRequested = useRef(false); const runId = useRef(0);
  const setGraph = (value: Graph) => { dispatch({ type: 'set', value }); setResult(null); };
  useEffect(() => () => { runId.current++; if (job.current) void client.cancelJob(job.current).catch(() => {}); }, [client]);
  const descriptor = (node: Node) => node.type === 'operation' ? operations.find(op => op.id === node.operation) : undefined;
  const title = (node: Node) => node.type === 'input' || node.type === 'output' ? node.name : node.type === 'operation' ? descriptor(node)?.name ?? node.operation : node.type === 'combine' ? node.combine.kind : node.type;
  const nodes: Card[] = graph.nodes.map((node, i) => ({ id: node.id, type: 'graph', position: graph.presentation?.[node.id] ?? { x: 40 + (i % 3) * 260, y: 60 + Math.floor(i / 3) * 190 }, selected: node.id === selected, measured: sizes[node.id], data: { domain: node, title: title(node), ports: ports(node, operations).map(p => p.id), status: running ? 'Running job' : result?.nodes.find(n => n.node_id === node.id)?.status ?? 'Idle' } }));
  const edges = graph.edges.map(edge => ({ id: edge.id, source: edge.source_node, sourceHandle: edge.source_port, target: edge.target_node, targetHandle: edge.target_port }));
  const selectedNode = graph.nodes.find(n => n.id === selected);
  const outputResult = result?.outputs.find(o => o.name === output) ?? result?.outputs[0];
  const outputBytes = useMemo(() => outputResult?.output_base64 != null ? parseBase64(outputResult.output_base64) : null, [outputResult]);
  const updateNode = (updated: Node) => setGraph({ ...graph, nodes: graph.nodes.map(n => n.id === updated.id ? updated : n) });
  const add = (node: WithoutId<Node>) => {
    const id = crypto.randomUUID(); setGraph({ ...graph, nodes: [...graph.nodes, { ...node, id } as Node] }); select(id);
  };
  const connect = (connection: Connection) => {
    if (!connection.source || !connection.target || !connection.targetHandle || !canConnect(graph, connection.source, connection.target, connection.targetHandle, operations)) { setError('Connection rejected: check port types, cycles, and existing bindings.'); return; }
    setGraph({ ...graph, edges: [...graph.edges, { id: crypto.randomUUID(), source_node: connection.source, source_port: 'data', target_node: connection.target, target_port: connection.targetHandle }] }); setError('');
  };
  const changes = (changes: NodeChange<Card>[]) => {
    let next = graph;
    for (const change of changes) {
      if (change.type === 'dimensions' && change.dimensions) { const dimensions = change.dimensions; setSizes(previous => ({ ...previous, [change.id]: dimensions })); }
      if (change.type === 'select' && change.selected) select(change.id);
      if (change.type === 'position' && change.position) next = { ...next, presentation: { ...next.presentation, [change.id]: change.position } };
      if (change.type === 'remove') {
        const presentation = { ...next.presentation }; delete presentation[change.id];
        next = { ...next, presentation, nodes: next.nodes.filter(n => n.id !== change.id), edges: next.edges.filter(e => e.source_node !== change.id && e.target_node !== change.id) };
      }
    }
    if (next !== graph) setGraph(next);
  };
  const request = () => ({ graph, inputs_base64: Object.fromEntries(graph.nodes.filter(n => n.type === 'input').map(n => {
    const value = values[n.id] ?? { value: '', mode: 'text' as const }; return [(n as Extract<Node, { type: 'input' }>).name, encodeBase64(parseInput(value.value, value.mode))];
  })), outputs: [] });
  const run = async () => {
    if (running) return;
    const id = ++runId.current; setRunning(true); setError(''); setResult(null); cancelRequested.current = false;
    try {
      const caps = await client.capabilities(); if (!caps.graphs) throw new Error('Engine does not support graphs.');
      setCanCancel(caps.jobs);
      const req = request(); await client.validateGraph(req);
      let response: GraphResult;
      if (caps.jobs) {
        const submitted = await client.submitJob({ method: 'graph_execute', params: req }); job.current = submitted.id;
        if (runId.current !== id) { await client.cancelJob(submitted.id); throw new Error('Graph view closed.'); }
        if (cancelRequested.current) await client.cancelJob(submitted.id);
        for (;;) {
          const state = await client.getJob(submitted.id);
          if (state.status === 'succeeded') {
            if (state.result?.kind !== 'graph') throw new Error('Job returned no graph result.');
            response = state.result.value; break;
          }
          if (['failed', 'cancelled', 'timed_out'].includes(state.status)) throw new Error(state.error?.message ?? `Graph ${state.status}`);
          await new Promise(resolve => setTimeout(resolve, 100));
        }
      } else response = await client.executeGraph(req);
      if (runId.current === id) { setResult(parseGraphResponse(response)); setOutput(response.outputs[0]?.name ?? ''); }
    } catch (error) { if (runId.current === id) setError(errorMessage(error)); }
    finally { job.current = null; if (runId.current === id) { setRunning(false); setCanCancel(false); } }
  };
  const cancel = async () => {
    cancelRequested.current = true;
    try { if (job.current) await client.cancelJob(job.current); } catch (error) { setError(errorMessage(error)); }
  };
  const load = async (file?: File) => {
    if (!file) return;
    setNotice('Validating graph…');
    try {
      if (file.size > 4 * 1024 * 1024) throw new Error('Graph definition exceeds 4 MiB.');
      const candidate = parseContract('graph_request', { graph: JSON.parse(await file.text()) });
      await client.validateGraph(candidate); setGraph(candidate.graph); setValues({}); select(candidate.graph.nodes[0]?.id ?? ''); setError(''); setNotice('Graph loaded. Bind input values before running.'); setFitRevision(v => v + 1);
    } catch (error) { setNotice(''); setError(errorMessage(error)); }
  };
  const save = () => {
    if (graph.nodes.some(n => n.type === 'operation' ? !descriptor(n) : n.type === 'recipe' && n.steps.some(step => !operations.some(op => op.id === step.op || op.name === step.op)))) {
      setError('Load the operation registry before exporting a graph.'); return;
    }
    const sensitive = graph.nodes.some(n => n.type === 'operation'
      ? Object.keys(n.args ?? {}).some(name => descriptor(n)?.args.some(arg => arg.name === name && arg.sensitive))
      : n.type === 'recipe' && n.steps.some(step => operations.find(op => op.id === step.op || op.name === step.op)?.args.some((arg, i) => arg.sensitive && step.args?.[i] !== undefined)));
    if (sensitive) { setError('Connect sensitive arguments to named inputs before saving. Graph exports exclude input payloads.'); return; }
    downloadBytes(new TextEncoder().encode(JSON.stringify(graph, null, 2)), `${graph.id}.rxgraph.json`);
  };
  return <section className="graph-workbench" aria-label="Graph workbench">
    <div className="graph-toolbar">
      <button onClick={() => add({ type: 'input', name: `input-${graph.nodes.filter(n => n.type === 'input').length + 1}` })}>Add input</button>
      <button onClick={() => add({ type: 'output', name: `output-${graph.nodes.filter(n => n.type === 'output').length + 1}` })}>Add output</button>
      <select aria-label="Graph operation" value={operation} onChange={e => setOperation(e.target.value)}>{operations.map(op => <option key={op.id} value={op.id}>{op.name}</option>)}</select>
      <button onClick={() => add({ type: 'operation', operation, args: {}, enabled: true })}>Add operation</button>
      <select aria-label="Combine kind" value={combine} onChange={e => setCombine(e.target.value)}>{['concatenate', 'compare', 'xor', 'join_text', 'interleave'].map(kind => <option key={kind}>{kind}</option>)}</select>
      <button onClick={() => add({ type: 'combine', combine: combine === 'concatenate' ? { kind: 'concatenate', inputs: 2 } : combine === 'join_text' ? { kind: 'join_text', inputs: 2, separator: '\n' } : { kind: combine as 'compare' | 'xor' | 'interleave' } })}>Add combine</button>
      <button onClick={() => void run()} disabled={running}>Run graph</button><button onClick={() => void cancel()} disabled={!running || !canCancel}>Cancel graph</button>
      <button onClick={() => dispatch({ type: 'undo' })} disabled={!history.past.length}>Undo graph</button><button onClick={() => dispatch({ type: 'redo' })} disabled={!history.future.length}>Redo graph</button>
      <button onClick={() => { setGraph({ ...graph, presentation: Object.fromEntries(graph.nodes.map((node, i) => [node.id, { x: 40 + (i % 4) * 260, y: 40 + Math.floor(i / 4) * 240 }])) }); setFitRevision(v => v + 1); }}>Arrange nodes</button><button onClick={save}>Save graph</button><label>Load graph<input aria-label="Graph file" type="file" accept=".json" onChange={e => void load(e.target.files?.[0])} /></label>
    </div>
    <div className="graph-connection"><label>Source<select aria-label="Connection source" value={source} onChange={e => setSource(e.target.value)}>{graph.nodes.filter(n => n.type !== 'output').map(n => <option value={n.id} key={n.id}>{title(n)} · {n.id}</option>)}</select></label>
      <label>Target<select aria-label="Connection target" value={target} onChange={e => { setTarget(e.target.value); setPort('data'); }}>{graph.nodes.filter(n => n.type !== 'input').map(n => <option value={n.id} key={n.id}>{title(n)} · {n.id}</option>)}</select></label>
      <label>Port<select aria-label="Connection port" value={port} onChange={e => setPort(e.target.value)}>{graph.nodes.find(n => n.id === target) && ports(graph.nodes.find(n => n.id === target)!, operations).map(p => <option key={p.id}>{p.id}</option>)}</select></label>
      <button onClick={() => connect({ source, target, sourceHandle: 'data', targetHandle: port })}>Connect nodes</button>
    </div>
    {notice && <p role="status">{notice}</p>}
    {error && <p role="alert">{error}</p>}
    <div className="graph-layout"><div className="graph-canvas"><ReactFlow colorMode={theme} nodes={nodes} edges={edges} nodeTypes={nodeTypes} onNodesChange={changes} onConnect={connect} isValidConnection={c => canConnect(graph, c.source, c.target, c.targetHandle ?? '', operations)} onEdgesDelete={deleted => setGraph({ ...graph, edges: graph.edges.filter(e => !deleted.some(d => d.id === e.id)) })} fitView><Background /><Controls /><MiniMap /></ReactFlow></div>
      <aside className="graph-inspector" aria-label="Graph inspector"><label>Selected node<select aria-label="Selected graph node" value={selected} onChange={e => select(e.target.value)}>{graph.nodes.map(n => <option key={n.id} value={n.id}>{title(n)} · {n.id}</option>)}</select></label><h3>{selectedNode ? title(selectedNode) : 'Select a node'}</h3>
        {selectedNode && <><p>{selectedNode.id}</p>{'name' in selectedNode && <label>Node name<input aria-label="Node name" value={selectedNode.name} onChange={e => updateNode({ ...selectedNode, name: e.target.value })} /></label>}
          {selectedNode.type === 'input' && <><label>Encoding<select aria-label="Graph input encoding" value={values[selected]?.mode ?? 'text'} onChange={e => setValues({ ...values, [selected]: { value: values[selected]?.value ?? '', mode: e.target.value as 'text' | 'hex' | 'base64' } })}><option>text</option><option>hex</option><option>base64</option></select></label><textarea aria-label="Graph input data" value={values[selected]?.value ?? ''} onChange={e => setValues({ ...values, [selected]: { mode: values[selected]?.mode ?? 'text', value: e.target.value } })} /><input type="file" aria-label="Graph input file" onChange={async e => { const file = e.target.files?.[0]; if (!file) return; try { if (file.size > (await client.capabilities()).max_input_bytes) throw new Error('Input exceeds engine limit.'); setValues({ ...values, [selected]: { mode: 'base64', value: encodeBase64(new Uint8Array(await file.arrayBuffer())) } }); } catch (error) { setError(errorMessage(error)); } }} /></>}
          {selectedNode.type === 'operation' && <><p>{descriptor(selectedNode)?.description}</p>{descriptor(selectedNode)?.args.map(arg => <label key={arg.name}>{arg.name}<input aria-label={`Graph argument ${arg.name}`} type={arg.sensitive ? 'password' : 'text'} placeholder={arg.default} value={selectedNode.args?.[arg.name] ?? ''} onChange={e => { const args = { ...selectedNode.args }; if (e.target.value === '') delete args[arg.name]; else args[arg.name] = e.target.value; updateNode({ ...selectedNode, args }); }} /></label>)}</>}
          {selectedNode.type === 'combine' && 'inputs' in selectedNode.combine && <label>Input ports<input aria-label="Combine input count" type="number" min="1" max="64" value={selectedNode.combine.inputs} onChange={e => { if ('inputs' in selectedNode.combine) updateNode({ ...selectedNode, combine: { ...selectedNode.combine, inputs: Math.max(1, Math.min(64, Number(e.target.value))) } }); }} /></label>}
          <button onClick={() => changes([{ type: 'remove', id: selected }])}>Delete node</button>
        </>}
        {result?.nodes.filter(n => n.node_id === selected).map(n => <p key={n.node_id}>{n.status} · {n.output_bytes} bytes · {n.elapsed_ms.toFixed(2)} ms{n.error && ` · ${n.error.message}`}</p>)}
        <h3>Connections</h3>{graph.edges.map(e => <button key={e.id} onClick={() => setGraph({ ...graph, edges: graph.edges.filter(edge => edge.id !== e.id) })}>Disconnect {e.source_node} → {e.target_node}:{e.target_port}</button>)}
      </aside></div>
    <div className="graph-outputs" aria-label="Graph outputs"><div role="tablist">{result?.outputs.map(o => <button role="tab" aria-selected={outputResult?.name === o.name} key={o.node_id} onClick={() => setOutput(o.name)}>{o.name}</button>)}</div>
      {outputResult && <><p>{outputResult.name}: {outputResult.output_bytes} bytes{outputResult.error && ` · ${outputResult.error.message}`}</p><select aria-label="Graph output view" value={view} onChange={e => setView(e.target.value as typeof view)}><option>hex</option><option>text</option><option>base64</option></select>
        <button disabled={!outputBytes} onClick={() => outputBytes && downloadBytes(outputBytes, `${outputResult.name}.bin`)}>Download graph output</button>
        <pre>{outputBytes ? view === 'hex' ? encodeHex(outputBytes.subarray(0, 4096)) : view === 'text' ? previewUtf8(outputBytes.subarray(0, 4096)) : encodeBase64(outputBytes.subarray(0, 4096)) : outputResult.error?.message}</pre>{outputBytes && outputBytes.length > 4096 && <p>Preview limited to 4096 bytes. Download contains the complete output.</p>}</>}
    </div>
  </section>;
}
