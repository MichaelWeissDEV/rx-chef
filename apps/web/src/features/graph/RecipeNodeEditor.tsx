import { useState } from 'react';
import type { OperationDescriptor, RecipeStep } from '../../types';
import { ArgumentControl } from './ArgumentControl';
import { loadRecipe, prepareRecipe } from '../../lib/recipe';
import { errorMessage } from '../../lib/contracts';
export function RecipeNodeEditor({ steps, operations, onChange, onError }: { steps: RecipeStep[]; operations: OperationDescriptor[]; onChange(steps: RecipeStep[]): void; onError(message: string): void }) {
  const [operation, choose] = useState('to_base64');
  return <section aria-label="Embedded recipe"><label>Recipe operation<select aria-label="Recipe node operation" value={operation} onChange={e => choose(e.target.value)}>{operations.map(op => <option key={op.id} value={op.id}>{op.name}</option>)}</select></label>
    <button onClick={() => { const descriptor = operations.find(op => op.id === operation); if (descriptor) onChange([...steps, { op: descriptor.id, args: descriptor.args.map(a => a.default) }]); }}>Add recipe step</button>
    <input aria-label="Embedded recipe file" type="file" accept=".json" onChange={async e => { const file = e.target.files?.[0]; if (!file) return; try { if (file.size > 4 << 20) throw new Error('Recipe exceeds size limit.'); onChange(prepareRecipe(loadRecipe(await file.text())).steps); } catch (error) { onError(errorMessage(error)); } }} />
    {steps.map((step, index) => { const descriptor = operations.find(op => op.id === step.op || op.name === step.op); return <details key={index} open><summary>{index + 1}. {descriptor?.name ?? step.op}</summary>
      {descriptor?.args.map((arg, argIndex) => <ArgumentControl key={arg.name} arg={arg} value={step.args?.[argIndex]} onChange={value => onChange(steps.map((s, i) => i === index ? { ...s, args: descriptor.args.map((a, j) => j === argIndex ? value : s.args?.[j] ?? a.default) } : s))} />)}
      <button onClick={() => onChange(steps.filter((_, i) => i !== index))}>Remove recipe step {index + 1}</button>
      {index > 0 && <button onClick={() => { const next = [...steps]; [next[index - 1], next[index]] = [next[index], next[index - 1]]; onChange(next); }}>Move recipe step up</button>}
    </details>; })}
  </section>;
}
