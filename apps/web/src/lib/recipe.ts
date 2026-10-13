import type { RecipeStep } from '../types';
const controls = new Set(['fork', 'merge', 'subsection', 'register', 'jump', 'conditionaljump', 'label', 'return']);
const canonical = (name: string) => name.replace(/[^a-z0-9]/gi, '').toLowerCase();
/** UI disable is omission only for ordinary operations; control changes must be explicit edits. */
export function prepareRecipe(recipe: RecipeStep[]) {
  for (const step of recipe) {
    if (step.disabled && controls.has(canonical(step.op))) throw new Error(`Cannot disable control-flow step ${step.op}. Edit the block explicitly.`);
  }
  const sourceIndices: number[] = [];
  const steps = recipe.flatMap((step, index) => {
    if (step.disabled) return [];
    sourceIndices.push(index);
    return [{ op: step.op, args: [...step.args] }];
  });
  return { steps, sourceIndices };
}
export function loadRecipe(text: string): RecipeStep[] {
  const value: unknown = JSON.parse(text);
  const steps = Array.isArray(value) ? value : (value as { steps?: unknown })?.steps;
  if (!Array.isArray(steps) || !steps.every(s => s && typeof s.op === 'string' &&
      (s.args === undefined || Array.isArray(s.args) && s.args.every((v: unknown) => typeof v === 'string')) &&
      (s.disabled === undefined || typeof s.disabled === 'boolean'))) throw new Error('Invalid recipe file.');
  return steps.map(s => ({ op: s.op, args: s.args ?? [], ...(s.disabled === undefined ? {} : { disabled: s.disabled }) }));
}
