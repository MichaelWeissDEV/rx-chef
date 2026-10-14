import { useEffect, useRef, useState } from 'react';
import { EditorState } from '@codemirror/state';
import { EditorView, keymap, lineNumbers, highlightActiveLine } from '@codemirror/view';
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
export function CodeEditor({ value, onChange }: { value: string; onChange: (text: string) => void }) {
  const host = useRef<HTMLDivElement>(null);
  const editor = useRef<EditorView | null>(null);
  const changed = useRef(onChange); changed.current = onChange;
  const [position, setPosition] = useState('Line 1 · Column 1');
  useEffect(() => {
    const view = new EditorView({ parent: host.current!, state: EditorState.create({ extensions: [
      lineNumbers(), highlightActiveLine(), history(), keymap.of([...defaultKeymap, ...historyKeymap]),
      EditorView.contentAttributes.of({ 'aria-label': 'Input data', role: 'textbox', 'aria-multiline': 'true' }),
      EditorView.updateListener.of(update => {
        if (update.docChanged) changed.current(update.state.doc.toString());
        if (update.selectionSet || update.docChanged) {
          const pos = update.state.selection.main.head; const line = update.state.doc.lineAt(pos);
          setPosition(`Line ${line.number} · Column ${pos - line.from + 1}`);
        }
      }), EditorView.theme({ '&': { height: '100%', background: 'var(--bg-primary)', color: 'var(--text-primary)' }, '.cm-scroller': { overflow: 'auto', fontFamily: 'var(--font-mono)' }, '.cm-gutters': { background: 'var(--bg-secondary)', color: 'var(--text-muted)', border: 'none' }, '.cm-cursor': { borderLeftColor: 'var(--text-primary)' } }),
    ] }) });
    editor.current = view;
    return () => { view.destroy(); editor.current = null; };
  }, []);
  useEffect(() => {
    const view = editor.current;
    if (view && view.state.doc.toString() !== value) view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value } });
  }, [value]);
  return <div className="code-editor"><div ref={host} className="editor-host" /><footer>{position}</footer></div>;
}
