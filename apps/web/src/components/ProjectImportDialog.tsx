import { useEffect, useRef } from 'react';
import type { WorkbenchProject } from '../types';
export function ProjectImportDialog({ project, onOpen, onClose }: { project: WorkbenchProject; onOpen(): void; onClose(): void }) {
  const dialog = useRef<HTMLDialogElement>(null);
  useEffect(() => { const element = dialog.current!; element.showModal(); return () => element.close(); }, []);
  return <dialog ref={dialog} className="project-export-dialog" aria-labelledby="project-import-title" onCancel={event => { event.preventDefault(); onClose(); }}>
    <header><div><small>REVIEW IMPORT</small><h2 id="project-import-title">Open {project.name}?</h2></div><button aria-label="Close import" onClick={onClose}>×</button></header>
    <p>This replaces your current workspace. Save or export your current work first if you want to keep it.</p>
    <div className="export-columns"><section><h3>Graphs · {project.graphs?.length ?? 0}</h3><ul>{project.graphs?.map(graph => <li key={graph.id}>{graph.id}</li>)}</ul></section><section><h3>Recipes · {project.recipes?.length ?? 0}</h3><ul>{project.recipes?.map(recipe => <li key={recipe.name}>{recipe.name}</li>)}</ul></section></div>
    <p>{project.history?.length ?? 0} history entries · {project.input_files?.length ?? 0} external file references</p>
    <p className="export-privacy">The file passed engine validation. Inputs will be unbound and results cleared. External files are never opened automatically.</p>
    <footer><button onClick={onClose}>Cancel import</button><button className="btn-primary" onClick={onOpen}>Open imported project</button></footer>
  </dialog>;
}
