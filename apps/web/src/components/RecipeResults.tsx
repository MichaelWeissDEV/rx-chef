import { useState } from 'react';
import type { ExecuteResponse } from '../types';
import { OutputPanel } from './OutputPanel';
export function RecipeResults(props: { response: ExecuteResponse | null; error: string | null; outputViewMode: 'text' | 'hex' | 'base64'; onChangeViewMode(mode: 'text' | 'hex' | 'base64'): void }) {
  const [kept, setKept] = useState<{ id: number; response: ExecuteResponse }[]>([]);
  const [selected, select] = useState(0); const [compare, setCompare] = useState(false);
  const [nextId, setNextId] = useState(1);
  const total = kept.reduce((sum, item) => sum + item.response.result.output_len, 0);
  const canKeep = Boolean(props.response) && !props.error && kept.length < 6 && total + (props.response?.result.output_len ?? 0) <= 16 * 1024 * 1024;
  const items = [{ id: 0, response: props.response }, ...kept];
  return <div className="recipe-results"><div className="result-controls"><div role="tablist" aria-label="Recipe result tabs">{items.map(item => <button role="tab" aria-selected={item.id === selected} key={item.id} onClick={() => select(item.id)}>{item.id ? `Result ${item.id}` : 'Live result'}</button>)}</div>
    <button disabled={!canKeep} onClick={() => { if (!props.response || !canKeep) return; setKept(previous => [...previous, { id: nextId, response: props.response! }]); setNextId(value => value + 1); }}>Keep recipe result</button>
    <label><input type="checkbox" checked={compare} onChange={event => setCompare(event.target.checked)} />Compare recipe results</label>
    <small>Kept in memory · up to 6 results / 16 MiB · excluded from project files</small></div>
    <div className={`result-cards ${compare ? 'compare' : ''}`}>{items.filter(item => compare || item.id === selected).map(item => <section key={item.id} aria-label={item.id ? `Result ${item.id}` : 'Live result'}>{item.id > 0 && <button onClick={() => { setKept(previous => previous.filter(saved => saved.id !== item.id)); if (selected === item.id) select(0); }}>Remove result {item.id}</button>}
      <OutputPanel {...props} response={item.response} error={item.id ? null : props.error} /></section>)}</div>
  </div>;
}
