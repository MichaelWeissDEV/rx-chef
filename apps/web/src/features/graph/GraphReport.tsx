import { useEffect, useRef, useState } from 'react';
import type { Graph, GraphResult, OperationDescriptor } from '../../types';
import { graphSvg, transformationSteps, type EdgeStyle } from './report';
import { downloadBytes, encodeBase64 } from '../../lib/bytes';
import { exportNativeFile } from '../../clients/tauri';
import { errorMessage } from '../../lib/contracts';
export async function exportGraphImage(graph: Graph, operations: OperationDescriptor[], result: GraphResult | null, style: EdgeStyle) {
  const bytes = new TextEncoder().encode(graphSvg(graph, operations, result, style));
  const filename = `dataflow-${graph.id.replace(/[^a-zA-Z0-9_-]/g, '-').slice(0, 80) || 'graph'}.svg`;
  if (window.__TAURI_INTERNALS__) return exportNativeFile(encodeBase64(bytes), filename);
  downloadBytes(bytes, filename); return true;
}
export function GraphReport({ graph, operations, result, style, onClose }: { graph: Graph; operations: OperationDescriptor[]; result: GraphResult | null; style: EdgeStyle; onClose(): void }) {
  const ref = useRef<HTMLDialogElement>(null); const [error, setError] = useState(''); const [busy, setBusy] = useState(false);
  useEffect(() => { const dialog = ref.current!; dialog.showModal(); return () => dialog.close(); }, []);
  return <dialog ref={ref} className="project-export-dialog transformation-dialog" aria-labelledby="transformation-title" onCancel={event => { event.preventDefault(); if (!busy) onClose(); }}>
    <header><h2 id="transformation-title">Transformation steps</h2><button aria-label="Close transformation steps" disabled={busy} onClick={onClose}>×</button></header>
    <p>{graph.id} · Dependency order; independent branches may run in a different order. Runtime values describe the current result. Intermediate graph payloads are not captured.</p>
    <ol className="transformation-steps">{transformationSteps(graph, operations, result).map(step => <li key={step.id}><strong>{step.title}</strong><small>{step.id}</small><p>{step.description}</p><ul>{step.details.map((detail, i) => <li key={i}>{detail}</li>)}</ul><p>{step.status}</p></li>)}</ol>
    <p>Input/output payloads are excluded and sensitive arguments are redacted. Review ordinary literal values before sharing. The SVG contains the full graph and this step list; fields longer than 1000 characters are shortened in the image.</p>
    {error && <p role="alert">{error}</p>}<footer><button disabled={busy} onClick={onClose}>Close steps</button><button disabled={busy} onClick={async () => { setBusy(true); try { await exportGraphImage(graph, operations, result, style); } catch (error) { setError(errorMessage(error)); } finally { setBusy(false); } }}>Export data flow SVG</button></footer>
  </dialog>;
}
