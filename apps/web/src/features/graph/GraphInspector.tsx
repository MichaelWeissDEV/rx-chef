import { useEffect, useRef } from 'react';
import type { Graph, GraphResult, Node, OperationDescriptor, RxchefClient, GraphNodeProgress } from '../../types';
import { ArgumentControl } from './ArgumentControl';
import { RecipeNodeEditor } from './RecipeNodeEditor';
import { encodeBase64 } from '../../lib/bytes';
import { errorMessage } from '../../lib/contracts';
export type InputValues = Record<string, { value: string; mode: 'text' | 'hex' | 'base64' }>;
type InputStamp={ticket:number;epoch:number;value:InputValues[string]|undefined;client:RxchefClient};
interface Props {
  graph: Graph; selected: string; select(id: string): void; title(node: Node): string; operations: OperationDescriptor[];
  values: InputValues; setValues(values: InputValues | ((current: InputValues) => InputValues)): void; updateNode(node: Node): void; remove(id: string): void;
  duplicate(id: string): void; result: GraphResult | null; client: RxchefClient; onError(message: string): void; disconnect(id: string): void;
  progress?: GraphNodeProgress[];
  inputStamp(id:string):InputStamp; inputCurrent(id:string,stamp:InputStamp):boolean;
}
export function GraphInspector({ graph, selected, select, title, operations, values, setValues, updateNode, remove, duplicate, result, client, onError, disconnect, progress, inputStamp,inputCurrent }: Props) {
  const panel = useRef<HTMLElement>(null);
  useEffect(() => { if (panel.current) panel.current.scrollTop = 0; }, [selected]);
  const node = graph.nodes.find(n => n.id === selected);
  const descriptor = node?.type === 'operation' ? operations.find(op => op.id === node.operation) : undefined;
  const kinds = Array.from(new Set(operations.flatMap(op => op.args.map(a => a.kind)))).sort();
  return <aside ref={panel} className="graph-inspector" aria-label="Graph inspector"><label>Selected node<select aria-label="Selected graph node" value={selected} onChange={e => select(e.target.value)}>{graph.nodes.map(n => <option key={n.id} value={n.id}>{title(n)} · {n.id}</option>)}</select></label><h3>{node ? title(node) : 'Select a node'}</h3>
    {node && <><details className="graph-node-id"><summary>Node identifier</summary><code>{node.id}</code></details><label>Display label<input aria-label="Node display label" value={node.label ?? ''} onChange={e => updateNode({ ...node, label: e.target.value || undefined })} /></label>
      {'name' in node && <label>Node name<input aria-label="Node name" value={node.name} onChange={e => updateNode({ ...node, name: e.target.value })} /></label>}
      {node.type === 'input' && <><label>Encoding<select aria-label="Graph input encoding" value={values[selected]?.mode ?? 'text'} onChange={e => setValues({ ...values, [selected]: { value: values[selected]?.value ?? '', mode: e.target.value as 'text' | 'hex' | 'base64' } })}><option>text</option><option>hex</option><option>base64</option></select></label>
        {!Object.hasOwn(values,selected) && <p role="note">Input is unbound. Enter data, select a file or bind an empty value.</p>}
        <button onClick={()=>setValues(current=>({...current,[selected]:{mode:'text',value:''}}))}>Bind empty input</button>
        <textarea aria-label="Graph input data" value={values[selected]?.value ?? ''} onChange={e => setValues({ ...values, [selected]: { mode: values[selected]?.mode ?? 'text', value: e.target.value } })} />
        <button onClick={async () => { const stamp=inputStamp(selected);try { const value = await navigator.clipboard.readText(); if(!inputCurrent(selected,stamp))throw new Error('Clipboard import discarded: graph or input changed.');setValues(current => ({ ...current, [selected]: { mode: 'text', value } })); } catch (error) { onError(errorMessage(error)); } }}>Paste clipboard</button>
        <input type="file" aria-label="Graph input file" onChange={async e => { const file = e.target.files?.[0]; e.target.value = ''; if (!file) return;const stamp=inputStamp(selected); try { if (file.size > (await client.capabilities()).max_input_bytes) throw new Error('Input exceeds engine limit.'); const value = encodeBase64(new Uint8Array(await file.arrayBuffer())); if(!inputCurrent(selected,stamp))throw new Error('File import discarded: graph or input changed.');setValues(current => ({ ...current, [selected]: { mode: 'base64', value } })); } catch (error) { onError(errorMessage(error)); } }} /></>}
      {node.type === 'operation' && <><p>{descriptor?.description.replace(/<[^>]*>/g, ' ')}</p><code>{node.operation}</code><label>Enabled<input type="checkbox" aria-label="Graph operation enabled" checked={node.enabled !== false} onChange={e => updateNode({ ...node, enabled: e.target.checked })} /></label>
        {descriptor?.args.map(arg => { const edge = graph.edges.find(e => e.target_node === node.id && e.target_port === arg.graph_port_id); return <ArgumentControl key={arg.name} arg={arg} value={node.args?.[arg.name]} bound={edge ? `${title(graph.nodes.find(n => n.id === edge.source_node)!)}:${edge.source_port}` : undefined} onChange={value => updateNode({ ...node, args: { ...node.args, [arg.name]: value } })} />; })}</>}
      {node.type === 'argument' && <label>Argument type<select aria-label="Argument node kind" value={node.kind} onChange={e => updateNode({ ...node, kind: e.target.value as typeof node.kind })}>{kinds.map(kind => <option key={kind}>{kind}</option>)}</select></label>}
      {node.type === 'recipe' && <RecipeNodeEditor steps={node.steps.map(s => ({ ...s, args: s.args ?? [] }))} operations={operations} onChange={steps => updateNode({ ...node, steps })} onError={onError} />}
      {node.type === 'combine' && <>
        {'inputs' in node.combine && <label>Input ports<input aria-label="Combine input count" type="number" min="1" max="64" value={node.combine.inputs} onChange={e => { if ('inputs' in node.combine) updateNode({ ...node, combine: { ...node.combine, inputs: Math.max(1, Math.min(64, Number(e.target.value))) } }); }} /></label>}
        {node.combine.kind === 'join_text' && <label>Separator<input aria-label="Join separator" value={node.combine.separator} onChange={e => { if (node.combine.kind === 'join_text') updateNode({ ...node, combine: { ...node.combine, separator: e.target.value } }); }} /></label>}
        {node.combine.kind === 'select' && <label>Selected input<input aria-label="Selected combine input" type="number" min="0" max={node.combine.inputs - 1} value={node.combine.selected} onChange={e => { if (node.combine.kind === 'select') updateNode({ ...node, combine: { ...node.combine, selected: Number(e.target.value) } }); }} /></label>}
      </>}
      <button onClick={() => duplicate(selected)}>Duplicate node</button><button onClick={() => remove(selected)}>Delete node</button>
    </>}
    {result?.nodes.filter(n => n.node_id === selected).map(n => <p key={n.node_id}>{n.status} · {n.output_bytes} bytes · {n.elapsed_ms.toFixed(2)} ms{n.error && ` · ${n.error.message}`}</p>)}
    {!result && progress?.filter(node=>node.node_id===selected).map(node=><p key={node.node_id} aria-label="Live graph node status">{node.state} · {node.output_bytes==null?'byte count pending':`${node.output_bytes} bytes`} · {node.elapsed_ms==null?'timing pending':`${node.elapsed_ms.toFixed(2)} ms`}{node.error&&` · ${node.error.message}`}</p>)}
    <h3>Connections</h3>{graph.edges.filter(e => e.source_node === selected || e.target_node === selected).map(e => <button key={e.id} onClick={() => disconnect(e.id)}>Disconnect {title(graph.nodes.find(n => n.id === e.source_node)!)} → {title(graph.nodes.find(n => n.id === e.target_node)!)}:{e.target_port}</button>)}
  </aside>;
}
