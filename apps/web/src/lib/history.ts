export interface History<T> { past: T[]; present: T; future: T[] }
export type HistoryAction<T> = { type: 'set'; value: T | ((previous: T) => T) } | { type: 'reset'; value: T } | { type: 'undo' | 'redo' };
export function historyReducer<T>(state: History<T>, action: HistoryAction<T>): History<T> {
  if (action.type === 'reset') return { past: [], present: action.value, future: [] };
  if (action.type === 'undo') return state.past.length ? { past: state.past.slice(0, -1), present: state.past.at(-1)!, future: [state.present, ...state.future] } : state;
  if (action.type === 'redo') return state.future.length ? { past: [...state.past, state.present], present: state.future[0], future: state.future.slice(1) } : state;
  if (action.type !== 'set') return state;
  const next = typeof action.value === 'function' ? (action.value as (previous: T) => T)(state.present) : action.value;
  if (JSON.stringify(next) === JSON.stringify(state.present)) return state;
  return { past: [...state.past.slice(-99), state.present], present: next, future: [] };
}
