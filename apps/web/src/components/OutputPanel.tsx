import React, { useMemo } from 'react';
import { Download, Copy, AlertTriangle } from 'lucide-react';
import { ExecuteResponse } from '../types';
import { parseBase64, encodeHex, previewUtf8, downloadBytes } from '../lib/bytes';

interface OutputPanelProps {
  response: ExecuteResponse | null;
  error: string | null;
  outputViewMode: 'text' | 'hex' | 'base64';
  onChangeViewMode: (mode: 'text' | 'hex' | 'base64') => void;
  elapsedMs?: number;
}

export const OutputPanel: React.FC<OutputPanelProps> = ({
  response,
  error,
  outputViewMode,
  onChangeViewMode,
  elapsedMs,
}) => {
  const decoded = useMemo(() => {
    try {
      const bytes = parseBase64(response?.result.output_base64 ?? '');
      if (response && bytes.length !== response.result.output_len) throw new Error('Result byte count mismatch.');
      return { bytes, error: null };
    } catch { return { bytes: new Uint8Array(), error: 'Invalid binary result from server.' }; }
  }, [response]);
  const bytes = decoded.bytes;
  error = error || decoded.error;
  const textOutput = useMemo(() => previewUtf8(bytes), [bytes]);
  // Render hex rows
  const hexRows = useMemo(() => {
    if (!bytes || bytes.length === 0) return [];
    const rows = [];
    const limit = Math.min(bytes.length, 16 * 1024); // Up to 16KB preview in hex table
    for (let i = 0; i < limit; i += 16) {
      const slice = bytes.slice(i, i + 16);
      const hexParts = [];
      let ascii = '';
      for (let j = 0; j < 16; j++) {
        if (j < slice.length) {
          const b = slice[j];
          hexParts.push(b.toString(16).padStart(2, '0'));
          ascii += b >= 32 && b <= 126 ? String.fromCharCode(b) : '.';
        } else {
          hexParts.push('  ');
        }
      }
      rows.push({
        offset: i.toString(16).padStart(8, '0'),
        hex: hexParts.join(' '),
        ascii,
      });
    }
    return rows;
  }, [bytes]);

  const handleDownload = () => { if (response && !error) downloadBytes(bytes); };

  return (
    <div className="panel" style={{ flex: 1, minWidth: '300px' }}>
      <div className="panel-header">
        <span>
          Output {response ? `(${response.result.output_len} bytes${elapsedMs ? `, ${elapsedMs.toFixed(1)}ms` : ''})` : ''}
        </span>
        <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
          <select
            value={outputViewMode}
            onChange={(e) => onChangeViewMode(e.target.value as 'text' | 'hex' | 'base64')}
            style={{ padding: '2px 6px', fontSize: '11px' }}
          >
            <option value="text">UTF-8 Text</option>
            <option value="hex">Hex Table</option>
            <option value="base64">Raw Base64</option>
          </select>
          <button className="btn-icon" disabled={!response || !!error} onClick={() => navigator.clipboard.writeText(textOutput)} title="Copy Text"><Copy size={13} />Text</button>
          <button className="btn-icon" disabled={!response || !!error} onClick={() => navigator.clipboard.writeText(encodeHex(bytes))} title="Copy HEX">HEX</button>
          <button className="btn-icon" disabled={!response || !!error} onClick={() => navigator.clipboard.writeText(response?.result.output_base64 ?? '')} title="Copy Base64">Base64</button>
          <button className="btn-icon" onClick={handleDownload} title="Download Raw Bytes" disabled={!response || !!error}>
            <Download size={13} />
          </button>
        </div>
      </div>

      {response && !response.result.output_is_utf8 && <p role="note">Text is a lossy preview. Raw download preserves the original bytes.</p>}
      <div className="panel-body">
        {error ? (
          <div style={{ padding: '16px', color: 'var(--color-danger)', display: 'flex', gap: '8px', alignItems: 'flex-start' }}>
            <AlertTriangle size={18} style={{ flexShrink: 0, marginTop: '2px' }} />
            <div>
              <div style={{ fontWeight: 600 }}>Execution Error</div>
              <div style={{ marginTop: '4px', fontFamily: 'var(--font-mono)', fontSize: '12px' }}>{error}</div>
            </div>
          </div>
        ) : outputViewMode === 'hex' ? (
          <div className="hex-view">
            {hexRows.map((r, idx) => (
              <div key={idx} className="hex-row">
                <span className="hex-offset">{r.offset}</span>
                <span className="hex-bytes">{r.hex}</span>
                <span className="hex-ascii">{r.ascii}</span>
              </div>
            ))}
            {bytes.length > 16 * 1024 && (
              <div style={{ padding: '8px', color: 'var(--text-muted)', textAlign: 'center' }}>
                --- Hex preview truncated to 16 KB ({bytes.length} bytes total) ---
              </div>
            )}
          </div>
        ) : outputViewMode === 'base64' ? (
          <textarea
            className="editor"
            value={response?.result?.output_base64 || ''}
            readOnly
            spellCheck={false}
          />
        ) : (
          <textarea
            className="editor"
            value={textOutput}
            readOnly
            placeholder="Result will appear here..."
            spellCheck={false}
          />
        )}
      </div>
    </div>
  );
};
