import { useMemo, useState } from 'react';
import type { GraphResult } from '../../types';
import { downloadBytes, encodeBase64, encodeHex, parseBase64, previewUtf8 } from '../../lib/bytes';
import { exportNativeFile } from '../../clients/tauri';
import { errorMessage } from '../../lib/contracts';
export function GraphOutputs({ result }: { result: GraphResult | null }) {
  const [name, select] = useState(''); const [view, choose] = useState<'text' | 'hex' | 'base64'>('hex');
  const output = result?.outputs.find(o => o.name === name) ?? result?.outputs[0];
  const bytes = useMemo(() => output?.output_base64 != null ? parseBase64(output.output_base64) : null, [output]);
  const preview = bytes?.subarray(0, 4096);
  const text = preview ? view === 'hex' ? encodeHex(preview) : view === 'text' ? previewUtf8(preview) : encodeBase64(preview) : output?.error?.message;
  const [copyError, setCopyError] = useState('');
  return <div className="graph-outputs" aria-label="Graph outputs"><div className="graph-output-heading"><strong>Results</strong>{!result && <span>Run the graph to inspect and download its outputs.</span>}</div><div role="tablist" aria-label="Named graph outputs">{result?.outputs.map(o => <button role="tab" aria-selected={output?.name === o.name} key={o.node_id} onClick={() => select(o.name)}>{o.name}</button>)}</div>
    {output && <><p>{output.name}: {output.output_bytes} bytes{output.error && ` · ${output.error.message}`}</p><select aria-label="Graph output view" value={view} onChange={e => choose(e.target.value as typeof view)}><option>hex</option><option>text</option><option>base64</option></select>
      <button disabled={!bytes} onClick={async () => { try { if (bytes) { if (window.__TAURI_INTERNALS__) await exportNativeFile(encodeBase64(bytes), `${output.name.replace(/[^a-zA-Z0-9_-]/g, '-') || 'output'}.bin`); else downloadBytes(bytes, `${output.name}.bin`); } setCopyError(''); } catch (error) { setCopyError(errorMessage(error)); } }}>Download graph output</button>
      <button disabled={!bytes} onClick={async () => { try { if (bytes) await navigator.clipboard.writeText(view === 'hex' ? encodeHex(bytes) : view === 'base64' ? encodeBase64(bytes) : new TextDecoder('utf-8', { fatal: true }).decode(bytes)); setCopyError(''); } catch { setCopyError('Cannot copy as text: select HEX/Base64 for binary data, or allow clipboard access.'); } }}>Copy graph output</button>
      {view === 'text' && <p role="note">Text preview may replace invalid UTF-8. Download preserves the original bytes.</p>}
      <pre>{text}</pre>{bytes && bytes.length > 4096 && <p>Preview limited to 4096 bytes. Download contains the complete output.</p>}{copyError && <p role="alert">{copyError}</p>}</>}
  </div>;
}
