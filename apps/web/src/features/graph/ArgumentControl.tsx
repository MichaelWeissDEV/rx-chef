import type { ArgumentDescriptor } from '../../types';
export function ArgumentControl({ arg, value, onChange, bound }: { arg: ArgumentDescriptor; value?: string; onChange(value: string): void; bound?: string }) {
  const label = `Graph argument ${arg.name}`;
  const displayed = bound ? '' : value ?? arg.default;
  return <label title={arg.description}>{arg.name}{arg.required && ' *'}{arg.sensitive && ' · sensitive'}
    {bound ? <p role="note">Bound from {bound}</p> : arg.choices.length ? <select aria-label={label} value={displayed} onChange={e => onChange(e.target.value)}>{arg.choices.map(choice => <option key={choice}>{choice}</option>)}</select>
      : arg.kind === 'boolean' ? <select aria-label={label} value={displayed} onChange={e => onChange(e.target.value)}><option>true</option><option>false</option></select>
      : <input aria-label={label} type={arg.sensitive ? 'password' : ['integer', 'unsigned_integer', 'float'].includes(arg.kind) ? 'number' : 'text'} step={arg.kind === 'float' ? 'any' : 1} min={arg.minimum ?? undefined} max={arg.maximum ?? undefined} value={displayed} placeholder={arg.default} onChange={e => onChange(e.target.value)} />}
  </label>;
}
