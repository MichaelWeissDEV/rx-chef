import { useRef } from 'react';
import type { RecipeStep } from '../types';
import { downloadBytes, encodeUtf8 } from '../lib/bytes';
import { loadRecipe } from '../lib/recipe';
import { errorMessage } from '../lib/contracts';
export function RecipeTools({ recipe, onLoad, onError, undo, redo, canUndo, canRedo }: {
  recipe: RecipeStep[]; onLoad: (steps: RecipeStep[]) => void; onError: (error: string) => void;
  undo: () => void; redo: () => void; canUndo: boolean; canRedo: boolean;
}) {
  const file = useRef<HTMLInputElement>(null);
  return <nav className="recipe-tools" aria-label="Recipe tools">
    <button className="btn btn-secondary" disabled={!canUndo} onClick={undo}>Undo</button>
    <button className="btn btn-secondary" disabled={!canRedo} onClick={redo}>Redo</button>
    <button className="btn btn-secondary" onClick={() => downloadBytes(encodeUtf8(JSON.stringify({ version: 1, steps: recipe }, null, 2)), 'recipe.json')}>Save recipe</button>
    <button className="btn btn-secondary" onClick={() => file.current?.click()}>Load recipe</button>
    <small>Recipe files include all arguments, including keys.</small>
    <input ref={file} type="file" aria-label="Recipe file" accept=".json" hidden onChange={async e => {
      const selected = e.target.files?.[0]; e.target.value = '';
      if (!selected) return;
      try { if (selected.size > 1024 * 1024) throw new Error('Recipe exceeds 1 MiB.'); onLoad(loadRecipe(await selected.text())); }
      catch (error) { onError(errorMessage(error)); }
    }} />
  </nav>;
}
