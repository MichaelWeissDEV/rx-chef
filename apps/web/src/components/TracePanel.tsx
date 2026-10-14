import { useState } from 'react';
import type { TraceArtifact } from '../types';
import { parseBase64, encodeHex, previewUtf8 } from '../lib/bytes';
export function TracePanel({ trace, truncated }: { trace: TraceArtifact[]; truncated: boolean }) {
  const [selected, setSelected] = useState(0);
  const [mode, setMode] = useState('hex');
  if (!trace.length) return null;
  const item = trace[Math.min(selected, trace.length - 1)];
  let preview: string;
  try { const bytes = parseBase64(item.output_base64); preview = mode === 'hex' ? encodeHex(bytes) : previewUtf8(bytes); } catch { preview = 'Invalid trace bytes'; }
  return <section className="trace-panel" aria-label="Trace inspector">
    <div className="panel-header"><span>Trace ({trace.length} entries){truncated && ' · truncated'}</span>
      <select aria-label="Trace view" value={mode} onChange={e => setMode(e.target.value)}><option value="hex">HEX</option><option value="text">Text preview</option></select>
    </div>
    <div className="trace-content"><div className="trace-list">{trace.map((t, i) => <button className={i === selected ? 'active' : ''} key={i} onClick={() => { setSelected(i); document.getElementById(`recipe-step-${t.step_index}`)?.focus(); }}>
      Step {t.step_index + 1}: {t.operation} · {t.input_bytes} → {t.output_bytes} B · {t.elapsed_ms.toFixed(2)} ms
    </button>)}</div><div><small>{item.truncated ? 'Prefix only; remaining bytes were not captured.' : 'Captured output bytes.'} {mode === 'text' && 'Text may be lossy.'}</small><pre>{preview}</pre></div></div>
  </section>;
}
