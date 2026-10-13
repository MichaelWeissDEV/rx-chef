import React from 'react';
import { TraceArtifact } from '../types';

interface TracePanelProps {
  trace: TraceArtifact[];
  truncated: boolean;
}

export const TracePanel: React.FC<TracePanelProps> = ({ trace, truncated }) => {
  if (trace.length === 0) return null;

  return (
    <div
      style={{
        height: '140px',
        backgroundColor: 'var(--bg-secondary)',
        borderTop: '1px solid var(--border-subtle)',
        display: 'flex',
        flexDirection: 'column',
        flexShrink: 0,
      }}
    >
      <div className="panel-header" style={{ height: '28px', fontSize: '11px' }}>
        <span>Trace & Step Inspector ({trace.length} steps logged{truncated ? ', Truncated' : ''})</span>
      </div>

      <div style={{ flex: 1, overflowY: 'auto', padding: '6px 12px', fontFamily: 'var(--font-mono)', fontSize: '11px' }}>
        {trace.map((t, idx) => (
          <div
            key={idx}
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              padding: '2px 0',
              borderBottom: '1px solid var(--border-subtle)',
            }}
          >
            <div style={{ display: 'flex', gap: '8px' }}>
              <span style={{ color: 'var(--text-muted)' }}>Step {t.step_index + 1}:</span>
              <span style={{ fontWeight: 600, color: 'var(--text-primary)' }}>{t.operation}</span>
              {t.control_flow && <span style={{ color: 'var(--accent-primary)' }}>[ControlFlow]</span>}
            </div>

            <div style={{ display: 'flex', gap: '16px', color: 'var(--text-secondary)' }}>
              <span>In: {t.input_bytes} B</span>
              <span>Out: {t.output_bytes} B</span>
              <span>{t.elapsed_ms.toFixed(2)} ms</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
