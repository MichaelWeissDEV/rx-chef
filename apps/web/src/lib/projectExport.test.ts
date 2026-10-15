import { describe, expect, it } from 'vitest';
import { exportOptions, prepareProjectExport, projectFilename, stableProjectText } from './projectExport';
import { starterGraph } from '../features/graph/model';
import type { WorkbenchProject } from '../types';
function project(): WorkbenchProject {
  return { version: 1, name: 'Team workspace', graphs: [{ ...starterGraph(), id: 'z' }, { ...starterGraph(), id: 'a' }],
    recipes: [{ name: 'z', steps: [{ op: 'To Base64', args: [] }, { op: 'From Base64', args: [] }] }, { name: 'a', steps: [] }],
    settings: { theme: 'light', selected_graph: 'z', selected_recipe: 'z' },
    input_files: [{ graph_id: 'z', input_name: 'input', path: '/private/file.bin', external: true }],
    history: [{ id: 'run', run_id: 'run-1', graph_id: 'z', started_at: '2026-10-15T00:00:00Z', status: 'succeeded', elapsed_ms: 1, output_sizes: { output: 3 } }] };
}
describe('GUI share/export selection', () => {
  it('defaults to definitions and layout without paths, history or personal settings', () => {
    const original = project(); const before = structuredClone(original);
    const result = prepareProjectExport(original, exportOptions(original));
    expect(result.graphs?.map(graph => graph.id)).toEqual(['a', 'z']);
    expect(result.recipes?.map(recipe => recipe.name)).toEqual(['a', 'z']);
    expect(result.settings).toEqual({}); expect(result.history).toEqual([]); expect(result.input_files).toEqual([]);
    expect(original).toEqual(before);
  });
  it('filters deselected definitions and their paths/history/selection, without rearranging recipe steps', () => {
    const source = project(); const options = { ...exportOptions(source, 'complete'), graphs: ['a'], recipes: ['z'], layout: false };
    const result = prepareProjectExport(source, options);
    expect(result.graphs).toHaveLength(1); expect(result.graphs![0].presentation).toEqual({});
    expect(result.settings).toEqual({ theme: 'light', selected_recipe: 'z' });
    expect(result.input_files).toEqual([]); expect(result.history).toEqual([]);
    expect(result.recipes![0].steps.map(step => step.op)).toEqual(['To Base64', 'From Base64']);
  });
  it('includes metadata only by explicit complete selection', () => {
    const source = project(); const result = prepareProjectExport(source, exportOptions(source, 'complete'));
    expect(result.history).toEqual(source.history); expect(result.input_files).toEqual(source.input_files); expect(result.settings).toEqual(source.settings);
  });
  it('does not spread unexpected credentials or payload fields into the project schema', () => {
    const source = Object.assign(project(), { token: 'PRIVATE-TOKEN', inputs_base64: { input: 'PRIVATE-BYTES' }, result: 'PRIVATE-RESULT' });
    const text = stableProjectText(prepareProjectExport(source, exportOptions(source, 'complete')));
    expect(text).not.toContain('PRIVATE-');
  });
  it('produces identical Git text for reordered objects and definition lists, with a final newline', () => {
    const a = project(); const b = structuredClone(a); b.graphs!.reverse(); b.recipes!.reverse();
    b.graphs!.forEach(graph => { graph.nodes.reverse(); graph.edges.reverse(); graph.presentation = Object.fromEntries(Object.entries(graph.presentation!).reverse()); });
    const text = stableProjectText(prepareProjectExport(a, exportOptions(a)));
    expect(text).toBe(stableProjectText(prepareProjectExport(b, exportOptions(b)))); expect(text.endsWith('\n')).toBe(true);
  });
  it('preserves unsupported versions for authoritative validation instead of silently downgrading', () => {
    const source = project(); source.version = 2;
    expect(prepareProjectExport(source, exportOptions(source)).version).toBe(2);
  });
  it.each(['../../escape', 'CON', 'LPT1', '...', 'folder/file', 'a\u0000b'])('suggests a portable filename for %s', name => {
    const filename = projectFilename(name);
    expect(filename).toMatch(/\.rxproject\.json$/); expect(filename).not.toMatch(/[\\/]/); expect(filename).not.toContain(String.fromCharCode(0));
    expect(filename).not.toMatch(/^(con|lpt1)\./i);
  });
});
