import { useMemo, useState } from 'react';
import { readPreferenceList } from '../../lib/preferences';
import type { OperationDescriptor } from '../../types';
function matches(text: string, query: string) {
  let position = 0; const source = text.toLocaleLowerCase();
  for (const character of query.toLocaleLowerCase()) { position = source.indexOf(character, position); if (position < 0) return false; position++; }
  return true;
}
export function GraphOperations({ operations, add, favorites, onToggleFavorite }: { operations: OperationDescriptor[]; favorites: string[]; onToggleFavorite(id: string): void; add(op: OperationDescriptor): void }) {
  const [query, search] = useState(''); const [category, choose] = useState('All');
  const [recent, setRecent] = useState(() => readPreferenceList('rxchef_recent_ops'));
  const [limit, more] = useState(80);
  const filtered = useMemo(() => {
    const needle = query.trim().toLowerCase();
    const score = (op: OperationDescriptor) => op.name.toLowerCase() === needle ? 0 : op.name.toLowerCase().startsWith(needle) ? 1 : op.name.toLowerCase().includes(needle) ? 2 : 3;
    return operations.filter(op => (!needle || matches(op.name, needle) || op.description.toLowerCase().includes(needle)) &&
      (category === 'All' || category === 'Favorites' ? category === 'All' || favorites.includes(op.id) || favorites.includes(op.name) : category === 'Recent' ? recent.includes(op.id) : op.module === category))
      .sort((a, b) => score(a) - score(b) || a.name.localeCompare(b.name));
  }, [operations, query, category, favorites, recent]);
  const addOperation = (op: OperationDescriptor) => { add(op); const next = [op.id, ...recent.filter(id => id !== op.id)].slice(0, 30); setRecent(next); try { localStorage.setItem('rxchef_recent_ops', JSON.stringify(next)); } catch { /* optional preferences */ } };
  return <aside className="graph-operations" aria-label="Graph operations"><div className="graph-operation-search"><h3>Find an operation</h3>
    <input type="search" aria-label="Search graph operations" placeholder="Search operations…" value={query} onChange={e => { search(e.target.value); more(80); }} onKeyDown={e => { if (e.key === 'Enter') { const op = filtered.find(op => op.graph_operation && op.availability === 'available'); if (op) addOperation(op); } }} />
    <select aria-label="Graph operation category" value={category} onChange={e => { choose(e.target.value); more(80); }}>{['All', 'Favorites', 'Recent', ...Array.from(new Set(operations.map(op => op.module))).sort()].map(name => <option key={name}>{name}</option>)}</select>
    <p>{filtered.length} matches · click to add</p></div>{filtered.slice(0, limit).map(op => <div key={op.id} className="graph-operation-row" draggable={op.graph_operation && op.availability === 'available'} onDragStart={e => e.dataTransfer.setData('application/rxchef-operation', op.id)}>
      <button disabled={!op.graph_operation || op.availability !== 'available'} title={op.graph_operation ? op.description.replace(/<[^>]*>/g, ' ') : 'Use this control-flow operation inside a Recipe node.'} onClick={() => addOperation(op)} aria-label={`Add graph operation ${op.name}`}>{op.name}<small>{op.module} · {op.args.length} parameters{op.availability !== 'available' && ' · unavailable'}{!op.graph_operation && ' · recipe only'}</small></button>
      <button aria-label={`Favorite ${op.name}`} aria-pressed={(favorites.includes(op.id) || favorites.includes(op.name))} onClick={() => onToggleFavorite(op.id)}>☆</button>
    </div>)}{limit < filtered.length && <button onClick={() => more(value => value + 80)}>Show more operations</button>}
  </aside>;
}
