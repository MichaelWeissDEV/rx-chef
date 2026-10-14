import { expect, it } from 'vitest';
import { loadRecipe, prepareRecipe } from './recipe';
it('retains source positions when ordinary steps are disabled', () => {
  expect(prepareRecipe([{ op: 'Reverse', args: [], disabled: true }, { op: 'To Hex', args: [] }])).toEqual({ steps: [{ op: 'To Hex', args: [] }], sourceIndices: [1] });
});
it.each(['Fork', 'Merge', 'Subsection', 'Register', 'Jump', 'Conditional Jump', 'Label', 'Return'])('rejects disabled control %s before execution', op => {
  expect(() => prepareRecipe([{ op, args: [], disabled: true }])).toThrow();
});
it('preserves enabled nested controls and register/jump arguments', () => {
  const steps = ['Fork', 'Register', 'Label', 'Jump', 'Merge'].map(op => ({ op, args: ['name'] }));
  expect(prepareRecipe(steps).steps).toEqual(steps);
});
it('imports legacy arrays and wrapped recipes without discarding disabled state', () => {
  expect(loadRecipe('[{"op":"Reverse"}]')).toEqual([{ op: 'Reverse', args: [] }]);
  expect(loadRecipe('{"steps":[{"op":"Reverse","args":[],"disabled":true}]}')[0].disabled).toBe(true);
  expect(() => loadRecipe('[{"op":"Reverse","args":[42]}]')).toThrow();
});
