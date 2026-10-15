import type { WorkbenchProject } from '../types';

export interface ProjectExportOptions {
  graphs: string[];
  recipes: string[];
  layout: boolean;
  settings: boolean;
  history: boolean;
  references: boolean;
}
export function exportOptions(project: WorkbenchProject, profile: 'team' | 'complete' = 'team'): ProjectExportOptions {
  return { graphs: (project.graphs ?? []).map(graph => graph.id), recipes: (project.recipes ?? []).map(recipe => recipe.name),
    layout: true, settings: profile === 'complete', history: profile === 'complete', references: profile === 'complete' };
}
const compare = (a: string, b: string) => a < b ? -1 : a > b ? 1 : 0;
/** Stable object keys and definition order make Git diffs useful; recipe step order is semantic. */
export function stableProjectText(project: WorkbenchProject): string {
  const stable = (value: unknown): unknown => Array.isArray(value) ? value.map(stable)
    : value && typeof value === 'object' ? Object.fromEntries(Object.entries(value).sort(([a], [b]) => compare(a, b)).map(([key, entry]) => [key, stable(entry)])) : value;
  return JSON.stringify(stable(project), null, 2) + '\n';
}
export function prepareProjectExport(project: WorkbenchProject, options: ProjectExportOptions): WorkbenchProject {
  const graphs = structuredClone(project.graphs ?? []).filter(graph => options.graphs.includes(graph.id)).sort((a, b) => compare(a.id, b.id));
  for (const graph of graphs) {
    graph.nodes.sort((a, b) => compare(a.id, b.id));
    graph.edges.sort((a, b) => compare(a.id, b.id));
    if (!options.layout) graph.presentation = {};
  }
  const recipes = structuredClone(project.recipes ?? []).filter(recipe => options.recipes.includes(recipe.name)).sort((a, b) => compare(a.name, b.name));
  const settings = options.settings ? structuredClone(project.settings ?? {}) : {};
  if (settings.selected_graph && !graphs.some(graph => graph.id === settings.selected_graph)) delete settings.selected_graph;
  if (settings.selected_recipe && !recipes.some(recipe => recipe.name === settings.selected_recipe)) delete settings.selected_recipe;
  // Deliberately construct the allowed schema, never spread arbitrary imported properties.
  return { version: project.version, name: project.name, graphs, recipes, settings,
    input_files: options.references ? structuredClone(project.input_files ?? []).filter(ref => graphs.some(graph => graph.id === ref.graph_id && graph.nodes.some(node => node.type === 'input' && node.name === ref.input_name))) : [],
    history: options.history ? structuredClone(project.history ?? []).filter(run => graphs.some(graph => graph.id === run.graph_id)) : [] };
}
export function projectFilename(name: string): string {
  let base = Array.from(name.normalize('NFC').replace(/[^\p{L}\p{N}._-]+/gu, '-').replace(/^[.-]+|[.-]+$/g, '')).slice(0, 80).join('') || 'project';
  if (/^(con|prn|aux|nul|com[0-9]|lpt[0-9])(?:\.|$)/i.test(base)) base = 'project-' + base;
  return base + '.rxproject.json';
}
