import { useEffect, useMemo, useRef, useState } from 'react';
import type { RxchefClient, WorkbenchProject } from '../types';
import { exportOptions, prepareProjectExport, projectFilename, stableProjectText } from '../lib/projectExport';
import { downloadBytes, encodeBase64 } from '../lib/bytes';
import { exportNativeFile } from '../clients/tauri';
import { errorMessage } from '../lib/contracts';

interface Props { project: WorkbenchProject; client: RxchefClient; onClose(): void; onExport(): void }
export function ProjectExportDialog({ project, client, onClose, onExport }: Props) {
  // Freeze the session at opening: later edits cannot silently change the shared file.
  const [snapshot] = useState(() => structuredClone(project));
  const [options, setOptions] = useState(() => exportOptions(snapshot));
  const [busy, setBusy] = useState(false); const [error, setError] = useState('');
  const [previewOpen, setPreviewOpen] = useState(false); const [validatedPreview, setValidatedPreview] = useState('');
  const dialog = useRef<HTMLDialogElement>(null);
  useEffect(() => { const element = dialog.current!; element.showModal(); return () => element.close(); }, []);
  const candidate = useMemo(() => prepareProjectExport(snapshot, options), [snapshot, options]);
  const text = useMemo(() => stableProjectText(candidate), [candidate]);
  useEffect(() => {
    if (!previewOpen) return; let current = true;
    setValidatedPreview(''); setError('');
    void client.validateProject(candidate).then(checked => { if (current) setValidatedPreview(stableProjectText(prepareProjectExport(checked, options))); }).catch(error => { if (current) setError(errorMessage(error)); });
    return () => { current = false; };
  }, [previewOpen, candidate, options, client]);
  const selected = options.graphs.length + options.recipes.length;
  const toggle = (kind: 'graphs' | 'recipes', id: string) => setOptions(previous => ({ ...previous, [kind]: previous[kind].includes(id) ? previous[kind].filter(value => value !== id) : [...previous[kind], id] }));
  const download = async () => {
    if (busy || !selected) return; setBusy(true); setError('');
    try {
      const checked = await client.validateProject(candidate);
      const bytes = new TextEncoder().encode(stableProjectText(prepareProjectExport(checked, options)));
      if (window.__TAURI_INTERNALS__) {
        const saved = await exportNativeFile(encodeBase64(bytes), projectFilename(snapshot.name));
        if (!saved) { setBusy(false); return; }
      } else downloadBytes(bytes, projectFilename(snapshot.name));
      onExport(); onClose();
    } catch (error) { setError(errorMessage(error)); setBusy(false); }
  };
  return <dialog ref={dialog} className="project-export-dialog" aria-labelledby="project-export-title" onCancel={event => { event.preventDefault(); if (!busy) onClose(); }}>
    <header><div><small>PORTABLE PROJECT</small><h2 id="project-export-title">Export & share project</h2></div><button aria-label="Close export" disabled={busy} onClick={onClose}>×</button></header>
    <p>Choose what travels with <strong>{snapshot.name}</strong>. The JSON file opens in both the desktop and web workbench.</p>
    <div className="export-presets"><button disabled={busy} onClick={() => setOptions(exportOptions(snapshot))}>Team / Git</button><button disabled={busy} onClick={() => setOptions(exportOptions(snapshot, 'complete'))}>Complete definitions</button></div>
    <div className="export-columns"><fieldset disabled={busy}><legend>Definitions</legend>
      <h3>Graphs</h3>{snapshot.graphs?.map(graph => <label key={graph.id}><input type="checkbox" checked={options.graphs.includes(graph.id)} onChange={() => toggle('graphs', graph.id)} />{graph.id}</label>)}
      <h3>Recipes</h3>{snapshot.recipes?.map(recipe => <label key={recipe.name}><input type="checkbox" checked={options.recipes.includes(recipe.name)} onChange={() => toggle('recipes', recipe.name)} />{recipe.name}</label>)}
    </fieldset><fieldset disabled={busy}><legend>Optional content</legend>{([
      ['layout', 'Node layout', 'Positions on the graph canvas.'], ['settings', 'Appearance & selection', 'Theme and selected definitions.'],
      ['history', 'Run history', 'Status, timings and sizes for included graphs.'], ['references', 'External file references', 'Paths only; may reveal local folder names. Files are not bundled.'],
    ] as const).map(([key, label, detail]) => <label key={key}><span><input type="checkbox" aria-label={label} aria-describedby={`export-${key}-detail`} checked={options[key]} onChange={event => setOptions({ ...options, [key]: event.target.checked })} />{label}</span><small id={`export-${key}-detail`}>{detail}</small></label>)}</fieldset></div>
    <p className="export-summary">{candidate.graphs?.length ?? 0} graphs · {candidate.recipes?.length ?? 0} recipes · {new TextEncoder().encode(text).length.toLocaleString()} bytes</p>
    <details onToggle={event => setPreviewOpen(event.currentTarget.open)}><summary>Preview export JSON</summary>{validatedPreview ? <pre aria-label="Project export preview">{validatedPreview}</pre> : <p>Validate selected definitions before displaying the preview.</p>}</details>
    <p className="export-privacy">Input/output bytes and connection credentials are never included. Known sensitive literals are rejected during validation. Review ordinary argument values before sharing.</p>
    <p>For Git: save this file in your repository, commit it with your usual Git tools, and import the updated file after pulling. Stable formatting keeps diffs readable. This does not push or merge automatically.</p>
    {!selected && <p role="note">Select at least one graph or recipe.</p>}{error && <p role="alert">{error}</p>}
    <footer><button disabled={busy} onClick={onClose}>Cancel export</button><button className="btn-primary" disabled={busy || !selected} onClick={() => void download()}>{busy ? 'Validating export…' : 'Download project file'}</button></footer>
  </dialog>;
}
