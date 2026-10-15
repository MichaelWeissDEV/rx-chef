import { useMemo, useState } from 'react';
import type { GraphResult } from '../../types';
import { downloadBytes, encodeBase64, encodeHex, parseBase64, previewUtf8 } from '../../lib/bytes';
import { exportNativeFile } from '../../clients/tauri';
import { errorMessage } from '../../lib/contracts';
import { DockPanel } from '../../components/DockPanel';
type Output = GraphResult['outputs'][number];
function OutputCard({ output }: { output: Output }) {
  const [view, choose] = useState<'text' | 'hex' | 'base64'>('hex');
  const bytes = useMemo(() => output.output_base64 != null ? parseBase64(output.output_base64) : null, [output]);
  const preview = bytes?.subarray(0, 4096);
  const text = preview ? view === 'hex' ? encodeHex(preview) : view === 'text' ? previewUtf8(preview) : encodeBase64(preview) : output.error?.message;
  const [copyError, setCopyError] = useState('');
  return <section className="graph-output-card" aria-label={`Output ${output.name}`}><p>{output.name}: {output.output_bytes} bytes{output.error && ` · ${output.error.message}`}</p><select aria-label="Graph output view" value={view} onChange={e => choose(e.target.value as typeof view)}><option>hex</option><option>text</option><option>base64</option></select>
    <button disabled={!bytes} onClick={async () => { try { if (bytes) { if (window.__TAURI_INTERNALS__) await exportNativeFile(encodeBase64(bytes), `${output.name.replace(/[^a-zA-Z0-9_-]/g, '-') || 'output'}.bin`); else downloadBytes(bytes, `${output.name}.bin`); } setCopyError(''); } catch (error) { setCopyError(errorMessage(error)); } }}>Download graph output</button>
    <button disabled={!bytes} onClick={async () => { try { if (bytes) await navigator.clipboard.writeText(view === 'hex' ? encodeHex(bytes) : view === 'base64' ? encodeBase64(bytes) : new TextDecoder('utf-8', { fatal: true }).decode(bytes)); setCopyError(''); } catch { setCopyError('Cannot copy as text: select HEX/Base64 for binary data, or allow clipboard access.'); } }}>Copy graph output</button>
    {view === 'text' && <p role="note">Text preview may replace invalid UTF-8. Download preserves the original bytes.</p>}
    <pre>{text}</pre>{bytes && bytes.length > 4096 && <p>Preview limited to 4096 bytes. Download contains the complete output.</p>}{copyError && <p role="alert">{copyError}</p>}
  </section>;
}
export function GraphOutputs({ result }: { result: GraphResult | null }) {
  const [name, select] = useState(''); const [compare, setCompare] = useState(false); const [hidden, hide] = useState<string[]>([]);
  const outputs = result?.outputs ?? []; const visible = outputs.filter(output => !hidden.includes(output.name));
  const output = visible.find(o => o.name === name) ?? visible[0];
  return <DockPanel title="Graph results" className="graph-result-dock"><div className="graph-outputs" aria-label="Graph outputs">
    {!result && <p>Run the graph to inspect and download its outputs.</p>}
    <div className="result-controls"><div role="tablist" aria-label="Named graph outputs">{visible.map(o => <button role="tab" aria-selected={output?.name === o.name} key={o.node_id} onClick={() => select(o.name)}>{o.name}</button>)}</div>
      {outputs.length > 1 && <label><input type="checkbox" checked={compare} onChange={event => setCompare(event.target.checked)} />Compare graph outputs</label>}
      {hidden.length > 0 && <button onClick={() => hide([])}>Show all graph outputs</button>}</div>
    <div className={`result-cards ${compare ? 'compare' : ''}`}>{(compare ? visible : output ? [output] : []).map(item => <div key={item.node_id}>
      {outputs.length > 1 && <button aria-label={`Hide output ${item.name}`} onClick={() => hide(previous => [...previous, item.name])}>Hide {item.name}</button>}<OutputCard output={item} /></div>)}</div>
  </div></DockPanel>;
}
