import { useEffect, useReducer, useRef, useState } from 'react';
import { ReactFlowProvider, useReactFlow, useNodesInitialized, type Connection, type NodeChange } from '@xyflow/react';
import { GraphCanvas, type GraphCardNode } from './GraphCanvas';
import '@xyflow/react/dist/style.css';
import type { Graph, Node, OperationDescriptor, RxchefClient, WorkbenchRun } from '../../types';
import { parseContract, errorMessage } from '../../lib/contracts';
import { encodeBase64, parseBase64, parseInput, downloadBytes } from '../../lib/bytes';
import { exportNativeFile, importNativeFile } from '../../clients/tauri';
import { historyReducer } from '../../lib/history';
import { ports, canConnect, starterGraph,requiredInputIds, executionKey, insertOperation, graphIssues, nextNodeName } from './model';
import { useGraphExecution } from './hooks/useGraphExecution';
import type { HistoryAction } from '../../lib/history';
import { GraphInspector } from './GraphInspector';
import { GraphOperations } from './GraphOperations';
import { GraphOutputs } from './GraphOutputs';
import './graph.css';
import { arrangeGraph } from './layout';

type WithoutId<T> = T extends unknown ? Omit<T, 'id'> : never;
type Card = GraphCardNode;
type Props = { client: RxchefClient; operations: OperationDescriptor[]; favorites: string[]; onToggleFavorite(id: string): void; theme: 'dark' | 'light'; visible: boolean; reservedIds?: string[]; document?: Graph; documentVersion?: number; projectEpoch?:number; onDocumentChange?(graph: Graph): void; onRun?(run: WorkbenchRun): void };
export function GraphEditor(props: Props) { return <ReactFlowProvider><Editor {...props} /></ReactFlowProvider>; }
function Editor({ client, operations, favorites, onToggleFavorite, theme, visible, reservedIds = [], document, documentVersion = 0, projectEpoch = 0, onDocumentChange, onRun }: Props) {
  const flow = useReactFlow<Card>();
  const initialized = useNodesInitialized();
  const toolbox = useRef<HTMLDetailsElement>(null);
  const [fitRevision, setFitRevision] = useState(0);
  const [sizes, setSizes] = useState<Record<string, { width: number; height: number }>>({});
  const [history, rawDispatch] = useReducer(historyReducer<Graph>, { past: [], present: document ?? starterGraph(), future: [] });
  const { state: execution, controller } = useGraphExecution(client);
  const result = execution.result;
  const recordedRun = useRef('');
  const runProjectEpoch = useRef(projectEpoch);
  useEffect(() => {
    const snapshot=execution.snapshot;
    if (!snapshot || !['succeeded','failed','cancelled','timed_out','discarded'].includes(execution.status) || recordedRun.current===snapshot.run_id) return;
    recordedRun.current=snapshot.run_id;
    if(runProjectEpoch.current!==projectEpoch) return;
    const failed=execution.status==='failed' || execution.result?.outputs.some(output=>output.error);
    onRun?.({id:crypto.randomUUID(),run_id:snapshot.run_id,graph_id:snapshot.request.graph.id,started_at:snapshot.started_at,
      status:failed?'failed':execution.status as WorkbenchRun['status'],elapsed_ms:Math.max(0,Date.now()-Date.parse(snapshot.started_at)),
      output_sizes:Object.fromEntries(execution.result?.outputs.map(output=>[output.name,output.output_bytes])??[]),
      error_code:failed?'operation':execution.status==='cancelled'?'cancelled':execution.status==='timed_out'?'timeout':null});
  }, [execution,onRun,projectEpoch]);
  const running = ['starting', 'queued', 'running'].includes(execution.status);
  const bindingEpoch=useRef(0); const currentClient=useRef(client);currentClient.current=client;
  const dispatch = (action: HistoryAction<Graph>) => { const next = historyReducer(history, action); if (executionKey(next.present) !== executionKey(history.present)) { bindingEpoch.current++; controller.editGraph(); } rawDispatch(action); };
  const graph = history.present;
  useEffect(() => {
    if (initialized && visible) requestAnimationFrame(() => { void flow.fitView({ padding: 0.15 }); });
  }, [initialized, fitRevision, flow, visible]);
  const [selected, rawSelect] = useState('input');
  const [selectedIds, setSelectedIds] = useState(['input']);
  const select = (id: string) => { rawSelect(id); setSelectedIds([id]); };
  const [fragment, setFragment] = useState<{nodes: Node[]; edges: Graph['edges']} | null>(null);
  const [values, rawSetValues] = useState<Record<string, { value: string; mode: 'text' | 'hex' | 'base64' }>>({});
  const [selectedOutputs,setSelectedOutputs] =useState<string[]>([]);
  const latestValues=useRef(values);
  const importTickets=useRef<Record<string,number>>({});
  const setValues = (next: typeof values | ((current: typeof values) => typeof values)) => { controller.editInputs();const updated=typeof next==='function'?next(latestValues.current):next;latestValues.current=updated; rawSetValues(updated); };
  const inputStamp=(id:string)=>{const ticket=(importTickets.current[id]??0)+1;importTickets.current[id]=ticket;return {ticket,epoch:bindingEpoch.current,value:latestValues.current[id],client:currentClient.current};};
  const inputCurrent=(id:string,stamp:ReturnType<typeof inputStamp>)=>stamp.ticket===importTickets.current[id]&&stamp.epoch===bindingEpoch.current&&stamp.value===latestValues.current[id]&&stamp.client===currentClient.current;
  const lastDocumentVersion = useRef(documentVersion);
  const pendingDocument = useRef<Graph | undefined>(undefined);
  useEffect(() => {
    if (lastDocumentVersion.current === documentVersion) return;
    lastDocumentVersion.current = documentVersion;
    if (document) { bindingEpoch.current++;pendingDocument.current=document; controller.editGraph(); controller.editInputs(); rawDispatch({ type:'reset', value:document }); latestValues.current={};rawSetValues({});setSelectedOutputs([]); select(document.nodes.find(node => node.type === 'input')?.id ?? document.nodes[0]?.id ?? ''); setFitRevision(v => v + 1); }
  }, [document, documentVersion, controller]);
  useEffect(() => { if(pendingDocument.current && pendingDocument.current!==graph)return; pendingDocument.current=undefined; onDocumentChange?.(graph); }, [graph, onDocumentChange]);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');

  const [source, setSource] = useState('input'); const [target, setTarget] = useState('output'); const [port, setPort] = useState('data');
  const activeSource = graph.nodes.find(node => node.id === source && node.type !== 'output')?.id ?? graph.nodes.find(node => node.type !== 'output')?.id ?? '';
  const activeTarget = graph.nodes.find(node => node.id === target && node.type !== 'input')?.id ?? graph.nodes.find(node => node.type !== 'input')?.id ?? '';
  const targetNode = graph.nodes.find(node => node.id === activeTarget);
  const targetPorts = targetNode ? ports(targetNode, operations) : [];
  const activePort = targetPorts.some(candidate => candidate.id === port) ? port : targetPorts[0]?.id ?? '';
  const [operation, setOperation] = useState('to_base64');
  const [automatic, setAutomatic] = useState(true);
  const loadTicket = useRef(0);
  const issues = graphIssues(graph, new Set(Object.keys(values)), selectedOutputs);
  useEffect(() => { if (!graph.nodes.some(node => node.id === selected)) select(graph.nodes.find(node => node.type === 'input')?.id ?? graph.nodes[0]?.id ?? ''); }, [graph.nodes, selected]);
  const [combine, setCombine] = useState('concatenate');
  const setGraph = (value: Graph) => dispatch({ type: 'set', value });
  const descriptor = (node: Node) => node.type === 'operation' ? operations.find(op => op.id === node.operation) : undefined;
  const title = (node: Node) => node.label || (node.type === 'input' || node.type === 'output' ? node.name : node.type === 'operation' ? descriptor(node)?.name ?? node.operation : node.type === 'combine' ? node.combine.kind : node.type === 'argument' ? `${node.kind} argument` : node.type);
  const nodes: Card[] = graph.nodes.map((node, i) => ({ id: node.id, type: 'graph', position: graph.presentation?.[node.id] ?? { x: 40 + (i % 3) * 260, y: 60 + Math.floor(i / 3) * 190 }, selected: selectedIds.includes(node.id), measured: sizes[node.id], data: { domain: node, title: title(node), category: descriptor(node)?.module ?? node.type, summary: node.type === 'operation' ? Object.entries(node.args ?? {}).map(([name, value]) => `${name}: ${descriptor(node)?.args.find(arg => arg.name === name)?.sensitive ? '••••' : value}`).join(' · ') : '', timing: result?.nodes.find(n => n.node_id === node.id) ? `${result.nodes.find(n => n.node_id === node.id)!.output_bytes} B · ${result.nodes.find(n => n.node_id === node.id)!.elapsed_ms.toFixed(1)} ms` : undefined, ports: ports(node, operations).map(p => p.id), status: result?.nodes.find(n => n.node_id === node.id)?.status ?? execution.nodes?.find(n=>n.node_id===node.id)?.state ?? (running ? 'Node status pending' : 'Idle') } }));
  const edges = graph.edges.map(edge => ({ id: edge.id, source: edge.source_node, sourceHandle: edge.source_port, target: edge.target_node, targetHandle: edge.target_port }));
  const updateNode = (updated: Node) => setGraph({ ...graph, nodes: graph.nodes.map(n => n.id === updated.id ? updated : n) });
  const add = (node: WithoutId<Node>) => {
    if (toolbox.current) toolbox.current.open = false;
    const id = crypto.randomUUID(); setGraph({ ...graph, nodes: [...graph.nodes, { ...node, id } as Node], presentation: { ...graph.presentation, [id]: { x: 40 + (graph.nodes.length % 3) * 280, y: 80 + Math.floor(graph.nodes.length / 3) * 180 } } }); select(id); setFitRevision(value => value + 1);
  };
  const addOperation = (id: string) => { if (toolbox.current) toolbox.current.open = false; const key = crypto.randomUUID(); setGraph(insertOperation(graph, id, key, selected, automatic)); select(key); setFitRevision(value => value + 1); };
  const connectionGraph = (connection: Connection, base = graph) => {
    const { source, target, targetHandle } = connection;
    if (!source || !target || !targetHandle) return null;
    const targetNode = base.nodes.find(n => n.id === target);
    const argument = targetNode?.type === 'operation' ? operations.find(op => op.id === targetNode.operation)?.args.find(a => a.graph_port_id === targetHandle) : undefined;
    const candidate = { ...base, edges: base.edges.filter(e => e.target_node !== target || e.target_port !== targetHandle),
      nodes: base.nodes.map(node => { if (node.id !== target || node.type !== 'operation' || !argument) return node; const args = { ...node.args }; delete args[argument.name]; return { ...node, args }; }) };
    return canConnect(candidate, source, target, targetHandle, operations) ? { ...candidate, edges: [...candidate.edges, { id: crypto.randomUUID(), source_node: source, source_port: 'data', target_node: target, target_port: targetHandle }] } : null;
  };
  const connect = (connection: Connection, base = graph) => {
    const next = connectionGraph(connection, base);
    if (!next) { setError('Connection rejected: check port types, cycles, and existing bindings.'); return; }
    setGraph(next); setError('');
  };
  const copyNodes = () => setFragment(structuredClone({ nodes: graph.nodes.filter(n => selectedIds.includes(n.id)), edges: graph.edges.filter(e => selectedIds.includes(e.source_node) && selectedIds.includes(e.target_node)) }));
  const pasteNodes = () => {
    if (!fragment) return;
    const ids = new Map(fragment.nodes.map(n => [n.id, crypto.randomUUID()]));
    const pasted = fragment.nodes.map(n => ({ ...structuredClone(n), id: ids.get(n.id)!, ...('name' in n ? { name: `${n.name}-copy-${ids.get(n.id)!.slice(0, 8)}` } : {}) }));
    setGraph({ ...graph, nodes: [...graph.nodes, ...pasted], edges: [...graph.edges, ...fragment.edges.map(e => ({ ...e, id: crypto.randomUUID(), source_node: ids.get(e.source_node)!, target_node: ids.get(e.target_node)! }))] }); setSelectedIds(pasted.map(n => n.id)); rawSelect(pasted[0]?.id ?? selected);
  };
  const changes = (changes: NodeChange<Card>[]) => {
    let next = graph;
    const selection = new Set(selectedIds);
    for (const change of changes) {
      if (change.type === 'dimensions' && change.dimensions) { const dimensions = change.dimensions; setSizes(previous => ({ ...previous, [change.id]: dimensions })); }
      if (change.type === 'select') { if (change.selected) { selection.add(change.id); rawSelect(change.id); } else selection.delete(change.id); }
      if (change.type === 'position' && change.position) next = { ...next, presentation: { ...next.presentation, [change.id]: change.position } };
      if (change.type === 'remove') {
        selection.delete(change.id);
        const presentation = { ...next.presentation }; delete presentation[change.id];
        next = { ...next, presentation, nodes: next.nodes.filter(n => n.id !== change.id), edges: next.edges.filter(e => e.source_node !== change.id && e.target_node !== change.id) };
      }
    }
    setSelectedIds([...selection]);
    if (next !== graph) setGraph(next);
  };
  const request = () => {
    if(selectedOutputs.some(id=>!graph.nodes.some(node=>node.id===id&&node.type==='output')))throw new Error('A selected output was removed. Choose outputs again.');
    const required=requiredInputIds(graph,selectedOutputs);
    return { graph, inputs_base64: Object.fromEntries(graph.nodes.filter(n => n.type === 'input'&&required.has(n.id)).map(n => {
    if(!Object.hasOwn(values,n.id)) throw new Error(`Bind input ${(n as Extract<Node,{type:'input'}>).name} before running.`);
    const value = values[n.id] ?? { value: '', mode: 'text' as const }; return [(n as Extract<Node, { type: 'input' }>).name, encodeBase64(parseInput(value.value, value.mode))];
  })), outputs: selectedOutputs.map(id=>(graph.nodes.find(node=>node.id===id) as Extract<Node,{type:'output'}>).name) };
  };
  const run = async () => {
    try { setError(''); runProjectEpoch.current=projectEpoch; await controller.start(request()); }
    catch (error) { setError(errorMessage(error)); }
  };
  const cancel = () => controller.cancel();
  const loadOwner = () => ({ ticket: ++loadTicket.current, epoch: bindingEpoch.current, client: currentClient.current });
  const load = async (file?: File, suppliedOwner?: ReturnType<typeof loadOwner>) => {
    if (!file) return;
    const owner = suppliedOwner ?? loadOwner();
    setNotice('Validating graph…');
    try {
      if (file.size > 4 * 1024 * 1024) throw new Error('Graph definition exceeds 4 MiB.');
      const candidate = parseContract('graph_request', { graph: JSON.parse(await file.text()) });
      if (reservedIds.includes(candidate.graph.id)) throw new Error('Graph name already exists in this project. Rename the imported graph before opening it here.');
      await owner.client.validateGraph(candidate);
      if (owner.ticket !== loadTicket.current || owner.epoch !== bindingEpoch.current || owner.client !== currentClient.current) throw new Error('Graph import discarded: graph, project or connection changed.');
      setGraph(candidate.graph); setValues({});setSelectedOutputs([]); select(candidate.graph.nodes.find(node => node.type === 'input')?.id ?? candidate.graph.nodes[0]?.id ?? ''); setError(''); setNotice('Graph loaded. Bind input values before running.'); setFitRevision(v => v + 1);
    } catch (error) { setNotice(''); setError(errorMessage(error)); }
  };
  const save = async () => {
    if (graph.nodes.some(n => n.type === 'operation' ? !descriptor(n) : n.type === 'recipe' && n.steps.some(step => !operations.some(op => op.id === step.op || op.name === step.op)))) {
      setError('Load the operation registry before exporting a graph.'); return;
    }
    const sensitive = graph.nodes.some(n => n.type === 'operation'
      ? Object.keys(n.args ?? {}).some(name => descriptor(n)?.args.some(arg => arg.name === name && arg.sensitive))
      : n.type === 'recipe' && n.steps.some(step => operations.find(op => op.id === step.op || op.name === step.op)?.args.some((arg, i) => arg.sensitive && step.args?.[i] !== undefined)));
    if (sensitive) { setError('Connect sensitive arguments to named inputs before saving. Graph exports exclude input payloads.'); return; }
    try {
      const bytes = new TextEncoder().encode(JSON.stringify(graph, null, 2));
      if (window.__TAURI_INTERNALS__) await exportNativeFile(encodeBase64(bytes), `${graph.id.replace(/[^a-zA-Z0-9_-]/g, '-') || 'graph'}.rxgraph.json`);
      else downloadBytes(bytes, `${graph.id}.rxgraph.json`);
    } catch (error) { setError(errorMessage(error)); }
  };
  return <section className="graph-workbench" aria-label="Graph workbench" tabIndex={0} onKeyDown={e => {
    if ((e.target as HTMLElement).closest('input, textarea, select, [contenteditable="true"]')) return;
    const command = e.ctrlKey || e.metaKey;
    if (command && e.key.toLowerCase() === 'z') { e.preventDefault(); dispatch({ type: e.shiftKey ? 'redo' : 'undo' }); }
    else if (command && e.key.toLowerCase() === 'c') { e.preventDefault(); copyNodes(); }
    else if (command && e.key.toLowerCase() === 'v') { e.preventDefault(); pasteNodes(); }
    else if (command && e.key === 'Enter') { e.preventDefault(); void run(); }
    else if (e.key === 'Delete' || e.key === 'Backspace') { e.preventDefault(); changes(selectedIds.map(id => ({ type: 'remove', id }))); }
  }}>
    <div className="graph-toolbar">
      <div className="graph-heading"><small>GRAPH WORKBENCH</small><strong>Connect your data, step by step</strong></div>
      <div className="graph-primary-actions"><button className="graph-run" disabled={running || !operations.length} onClick={() => void run()}>Run graph</button><button onClick={() => void cancel()} disabled={!running || !execution.canCancel}>Cancel graph</button><button onClick={() => void save()}>Save graph</button></div>
    </div>
    <div className="graph-document-tools">
      <label>Graph name<input aria-label="Graph name" value={graph.id} onChange={e => { if (reservedIds.includes(e.target.value)) { setError('Graph name already exists in this project. Choose another name.'); return; } setError(''); setGraph({ ...graph, id:e.target.value }); }} /></label>
      <details className="graph-output-picker"><summary>{selectedOutputs.length ? `${selectedOutputs.length} outputs selected` : 'Run all outputs'}</summary><fieldset aria-label="Graph output selection">
        <label><input type="checkbox" checked={!selectedOutputs.length} onChange={() => { controller.editGraph(); setSelectedOutputs([]); }} />All outputs</label>
        <small>Or choose specific outputs:</small>{graph.nodes.filter(node => node.type === 'output').map(node => <label key={node.id}><input type="checkbox" checked={selectedOutputs.includes(node.id)} onChange={event => { controller.editGraph(); setSelectedOutputs(previous => event.target.checked ? [...previous, node.id] : previous.filter(id => id !== node.id)); }} />{node.name}</label>)}
      </fieldset></details>
      <button onClick={() => add({ type: 'input', name: nextNodeName(graph, 'input') })}>Add input</button>
      <button onClick={() => add({ type: 'output', name: nextNodeName(graph, 'output') })}>Add output</button>
      <details ref={toolbox} className="graph-toolbox"><summary>More node types</summary><div className="graph-toolbox-content">
      <select aria-label="Graph operation" value={operation} onChange={e => setOperation(e.target.value)}>{operations.filter(op => op.graph_operation).map(op => <option key={op.id} value={op.id}>{op.name}</option>)}</select>
      <button onClick={() => addOperation(operation)}>Add operation</button>
      <select aria-label="Combine kind" value={combine} onChange={e => setCombine(e.target.value)}>{['concatenate', 'compare', 'xor', 'join_text', 'interleave', 'select'].map(kind => <option key={kind}>{kind}</option>)}</select>
      <button onClick={() => add({ type: 'combine', combine: combine === 'concatenate' ? { kind: 'concatenate', inputs: 2 } : combine === 'select' ? { kind: 'select', inputs: 2, selected: 0 } : combine === 'join_text' ? { kind: 'join_text', inputs: 2, separator: '\n' } : { kind: combine as 'compare' | 'xor' | 'interleave' } })}>Add combine</button>
      <button onClick={() => add({ type: 'argument', kind: 'bytes' })}>Add argument</button>
      <button onClick={() => add({ type: 'recipe', steps: [] })}>Add recipe</button>
      </div></details>
      <button onClick={copyNodes}>Copy nodes</button><button onClick={pasteNodes} disabled={!fragment?.nodes.length}>Paste nodes</button>
      <button onClick={() => dispatch({ type: 'undo' })} disabled={!history.past.length}>Undo graph</button><button onClick={() => dispatch({ type: 'redo' })} disabled={!history.future.length}>Redo graph</button>
      <button onClick={() => { setGraph({ ...graph, presentation: arrangeGraph(graph, sizes) }); setFitRevision(v => v + 1); }}>Arrange nodes</button><label className="graph-open-file">Open graph<input className="visually-hidden" aria-label="Graph file" type="file" accept=".json" onChange={e => { const file = e.target.files?.[0]; e.target.value = ''; void load(file); }} /></label>
      {Boolean(window.__TAURI_INTERNALS__) && <button onClick={async () => { try { const owner = loadOwner(); const data = await importNativeFile(); if (data !== null) await load(new File([new TextDecoder('utf-8', {fatal:true}).decode(parseBase64(data))], 'graph.rxgraph.json'), owner); } catch (error) { setError(errorMessage(error)); } }}>Open native graph file</button>}
    </div>
    <div className="graph-guide"><div><strong>1 · Bind inputs</strong><small>Choose an input node and enter data.</small></div><div><strong>2 · Add operations</strong><small>Click an operation to insert it into the selected chain.</small></div><div><strong>3 · Run & export</strong><small>Results appear below; downloads keep every byte.</small></div></div>
    <div className="graph-readiness"><label><input type="checkbox" checked={automatic} onChange={event => setAutomatic(event.target.checked)} />Connect new operations automatically</label><span>{graph.nodes.length} nodes · {graph.edges.length} connections</span>{issues.length ? issues.map(issue => <button key={issue.node} onClick={() => select(issue.node)}>{issue.message} →</button>) : <span className="graph-ready">Inputs ready · run to validate</span>}</div>
    <details className="graph-wiring"><summary>Connect nodes manually</summary><div className="graph-connection"><label>Source<select aria-label="Connection source" value={activeSource} onChange={e => setSource(e.target.value)}>{graph.nodes.filter(n => n.type !== 'output').map(n => <option value={n.id} key={n.id}>{title(n)} · {n.id}</option>)}</select></label>
      <label>Target<select aria-label="Connection target" value={activeTarget} onChange={e => { setTarget(e.target.value); setPort('data'); }}>{graph.nodes.filter(n => n.type !== 'input').map(n => <option value={n.id} key={n.id}>{title(n)} · {n.id}</option>)}</select></label>
      <label>Port<select aria-label="Connection port" value={activePort} onChange={e => setPort(e.target.value)}>{targetPorts.map(p => <option key={p.id}>{p.id}</option>)}</select></label>
      <button disabled={!activeSource || !activeTarget || !activePort} onClick={() => connect({ source: activeSource, target: activeTarget, sourceHandle: 'data', targetHandle: activePort })}>Connect nodes</button>
    </div></details>
    {(execution.notice || notice) && <p role="status">{execution.notice || notice}</p>}
    <div className="graph-run-summary"><p aria-label="Graph run status">{execution.status === 'idle' ? 'Ready when you are' : execution.status}{execution.cancellationRequested && running && ' · stopping earlier work…'}</p>{execution.snapshot && <details><summary>Run details</summary><code>{execution.snapshot.run_id} · revision {execution.snapshot.graph_revision}</code></details>}</div>
    {(error || execution.error) && <p role="alert">{error || execution.error}</p>}
    <div className="graph-layout"><GraphOperations operations={operations} favorites={favorites} onToggleFavorite={onToggleFavorite} add={op => addOperation(op.id)} /><GraphCanvas theme={theme} nodes={nodes} edges={edges} changes={changes} connect={connect} valid={c => Boolean(connectionGraph({ source: c.source, target: c.target, sourceHandle: c.sourceHandle ?? 'data', targetHandle: c.targetHandle ?? 'data' }))}
      deleteEdges={deleted => setGraph({ ...graph, edges: graph.edges.filter(e => !deleted.some(d => d.id === e.id)) })} reconnect={(edge, connection) => connect(connection, { ...graph, edges: graph.edges.filter(e => e.id !== edge.id) })}
      drop={e => { e.preventDefault(); const op = operations.find(op => op.id === e.dataTransfer.getData('application/rxchef-operation')); if (!op?.graph_operation || op.availability !== 'available') return; const id = crypto.randomUUID(); setGraph({ ...graph, nodes: [...graph.nodes, { id, type: 'operation', operation: op.id, args: {}, enabled: true }], presentation: { ...graph.presentation, [id]: flow.screenToFlowPosition({ x: e.clientX, y: e.clientY }) } }); select(id); }} />
      <GraphInspector graph={graph} selected={selected} select={select} title={title} operations={operations} values={values} setValues={setValues} updateNode={updateNode}
        inputStamp={inputStamp} inputCurrent={inputCurrent}
        remove={id => changes([{ type: 'remove', id }])} duplicate={id => { const node = graph.nodes.find(n => n.id === id); if (!node) return; const copy = structuredClone(node); const newId = crypto.randomUUID(); if ('name' in copy) copy.name += `-copy-${newId.slice(0, 8)}`; setGraph({ ...graph, nodes: [...graph.nodes, { ...copy, id: newId }], presentation: { ...graph.presentation, [newId]: { x: (graph.presentation?.[id]?.x ?? 40) + 40, y: (graph.presentation?.[id]?.y ?? 40) + 40 } } }); select(newId); }}
        result={result} progress={execution.nodes} client={client} onError={setError} disconnect={id => setGraph({ ...graph, edges: graph.edges.filter(e => e.id !== id) })} />
    </div>
    <GraphOutputs result={result} />
  </section>;
}
