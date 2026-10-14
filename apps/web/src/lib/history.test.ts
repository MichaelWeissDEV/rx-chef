import { expect, it } from 'vitest';
import { historyReducer } from './history';
it('undo/redo restores snapshots and new edits invalidate redo', () => {
  let state = { past: [] as number[], present: 0, future: [] as number[] };
  state = historyReducer(state, { type: 'set', value: 1 });
  state = historyReducer(state, { type: 'undo' });
  expect(state.present).toBe(0);
  state = historyReducer(state, { type: 'redo' });
  expect(state.present).toBe(1);
  state = historyReducer(state, { type: 'undo' });
  state = historyReducer(state, { type: 'set', value: 2 });
  expect(historyReducer(state, { type: 'redo' }).present).toBe(2);
});
it('opening another document clears history from the previous document', () => {
  const state=historyReducer({past:['old'],present:'current',future:['future']},{type:'reset',value:'new'});
  expect(historyReducer(state,{type:'undo'})).toEqual({past:[],present:'new',future:[]});
});
