import { useEffect, useRef, useState } from 'react';
import type { RxchefClient, WorkbenchProject } from '../types';
import { listProjects, saveProject, deleteProject } from '../lib/projectLibrary';
import { errorMessage, parseContract } from '../lib/contracts';
import { parseBase64 } from '../lib/bytes';
import { importNativeFile } from '../clients/tauri';
import { stableProjectText } from '../lib/projectExport';
import { ProjectImportDialog } from './ProjectImportDialog';
import { ProjectExportDialog } from './ProjectExportDialog';
interface Props { project: WorkbenchProject; client: RxchefClient; onOpen(project: WorkbenchProject): void; onRename(name: string): void; onSelectGraph(id: string): void; onSelectRecipe(name: string): void; onAddGraph(): void; onAddRecipe(): void }
export function ProjectPanel({ project, client, onOpen, onRename, onSelectGraph, onSelectRecipe, onAddGraph, onAddRecipe }: Props) {
  const [savedText, setSavedText] = useState('');
  const [panelOpen, setPanelOpen] = useState(false);
  const [imported, setImported] = useState<WorkbenchProject | null>(null);
  const [exporting, setExporting] = useState(false);
  const latest = useRef({ project, client }); latest.current = { project, client };
  const name=project.name; const [items, setItems] = useState<WorkbenchProject[]>([]); const [selected, select] = useState('');
  const [message, setMessage] = useState(''); const [error, setError] = useState(''); const [busy, setBusy] = useState(false);
  useEffect(() => { void listProjects().then(setItems).catch(error => setError(errorMessage(error))); }, []);
  const refresh = async () => setItems(await listProjects());
  const action = async (run: () => Promise<void>) => { if (busy) return; setBusy(true); setError(''); setMessage(''); try { await run(); } catch (error) { setError(errorMessage(error)); } finally { setBusy(false); } };
  const canonical = () => client.validateProject({ ...project, name });
  const load = async (data: unknown, owner = latest.current, review = false) => { const checked = await owner.client.validateProject(parseContract('workbench_project', data)); if (latest.current.project !== owner.project || latest.current.client !== owner.client) throw new Error('Project import discarded: the current project or connection changed.'); if (review) { setImported(checked); return; } onOpen(checked); setMessage('Project restored. Inputs are unbound; select files or enter values again.'); };
  const importFile = async (file?: File) => { if (!file) return; await action(async () => { if (file.size > 16 * 1024 * 1024) throw new Error('Project exceeds 16 MiB.'); const owner = latest.current; await load(JSON.parse(await file.text()), owner, true); }); };
  return <><div className="project-strip"><div><strong>{name || 'Untitled project'}</strong><small>{project.graphs?.length ?? 0} graphs · {project.recipes?.length ?? 0} recipes · {savedText === stableProjectText(project) ? 'Saved locally' : 'Unsaved changes'}</small></div><div>
    <button disabled={busy} onClick={() => void action(async () => { const savedSnapshot = stableProjectText(project); const checked = await canonical(); await saveProject(checked); setSavedText(savedSnapshot); await refresh(); setMessage('Project saved locally without input payloads or credentials.'); })}>Save project locally</button>
    <button disabled={busy} onClick={() => setExporting(true)}>Export project</button>
    </div></div><details className="project-panel" onToggle={event => setPanelOpen(event.currentTarget.open)}><summary>Project files & library</summary>
    <label>Project name<input aria-label="Project name" value={name} maxLength={256} onChange={e => onRename(e.target.value)} /></label>
    <label>Import project<input aria-label="Project file" type="file" accept=".json" disabled={busy} onChange={e => { const file = e.target.files?.[0]; e.target.value = ''; void importFile(file); }} /></label>
    {Boolean(window.__TAURI_INTERNALS__) && <button disabled={busy} onClick={() => void action(async () => { const owner = latest.current; const data = await importNativeFile(16 * 1024 * 1024); if (data !== null) await load(JSON.parse(new TextDecoder('utf-8', { fatal:true }).decode(parseBase64(data))), owner, true); })}>Open native project file</button>}
    <label>Saved projects<select aria-label="Saved projects" value={selected} onChange={e => select(e.target.value)}><option value="">Choose a saved project</option>{items.map(item => <option key={item.name}>{item.name}</option>)}</select></label>
    <button disabled={busy || !selected} onClick={() => void action(async () => { const item = items.find(item => item.name === selected); if (item) await load(item); })}>Open saved project</button>
    <button disabled={busy || !selected} onClick={() => void action(async () => { await deleteProject(selected); select(''); await refresh(); setMessage('Saved project removed from this device.'); })}>Delete saved project</button>
    <label>Project graphs<select aria-label="Project graphs" value={project.settings?.selected_graph ?? ''} onChange={e => onSelectGraph(e.target.value)}><option value="" disabled>Choose a graph</option>{project.graphs?.map(graph => <option key={graph.id}>{graph.id}</option>)}</select></label><button onClick={onAddGraph}>New project graph</button>
    <label>Project recipes<select aria-label="Project recipes" value={project.settings?.selected_recipe ?? ''} onChange={e => onSelectRecipe(e.target.value)}><option value="" disabled>Choose a recipe</option>{project.recipes?.map(recipe => <option key={recipe.name}>{recipe.name}</option>)}</select></label><button onClick={onAddRecipe}>New project recipe</button>
    <p>Inputs, outputs and connection tokens stay in memory. {(project.input_files?.length ?? 0)} external file reference(s) require rebinding. History contains status and sizes only.</p>
    <details><summary>Graph run history ({project.history?.length ?? 0})</summary><ul>{project.history?.slice(-20).reverse().map(run=><li key={run.id}>{run.run_id} · {run.graph_id} · {run.status} · {run.elapsed_ms.toFixed(0)} ms · {Object.entries(run.output_sizes??{}).map(([name,bytes])=>`${name}: ${bytes} B`).join(', ')}</li>)}</ul></details>
    {message && <p role="status">{message}</p>}{error && <p role="alert">{error}</p>}
  </details>{!panelOpen && (message || error) && <div className="project-feedback">{message && <p role="status">{message}</p>}{error && <p role="alert">{error}</p>}</div>}{imported && <ProjectImportDialog project={imported} onClose={() => setImported(null)} onOpen={() => { onOpen(imported); setImported(null); setMessage('Project restored. Inputs are unbound; select files or enter values again.'); }} />}{exporting && <ProjectExportDialog project={project} client={client} onClose={() => setExporting(false)} onExport={() => setMessage('Project exported. External files are references and are not included.')} />}</>;
}
