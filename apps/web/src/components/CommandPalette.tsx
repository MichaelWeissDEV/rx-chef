import { useState } from 'react';
import type { OperationDescriptor } from '../types';
export function CommandPalette({ operations, onSelect, onClose }: { operations: OperationDescriptor[]; onSelect: (op: OperationDescriptor) => void; onClose: () => void }) {
  const [query, setQuery] = useState('');
  const results = operations.filter(op => op.name.toLowerCase().includes(query.toLowerCase())).slice(0, 30);
  const select = (op: OperationDescriptor) => { onSelect(op); onClose(); };
  return <div className="modal-backdrop" onClick={onClose}><section className="command-palette" role="dialog" aria-modal="true" aria-label="Command palette" onClick={e => e.stopPropagation()} onKeyDown={e => { if (e.key === 'Escape') onClose(); }}>
    <input autoFocus aria-label="Find operation" value={query} onChange={e => setQuery(e.target.value)} onKeyDown={e => { if (e.key === 'Enter' && results[0]) select(results[0]); }} placeholder="Add an operation…" />
    {results.map(op => <button className="palette-result" aria-label={`Add ${op.name}`} key={op.id} onClick={() => select(op)}>{op.name}<small>{op.module} · {op.availability}</small></button>)}
    <button className="btn btn-secondary" onClick={onClose}>Close</button>
  </section></div>;
}
