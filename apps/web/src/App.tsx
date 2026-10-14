import React, { useState, useEffect, useMemo, useCallback, useReducer } from 'react';
import { HttpRxchefClient } from './clients/http';
import { TauriRxchefClient, configureConnection, importNativeFile } from './clients/tauri';
import { OperationDescriptor, RecipeStep, ExecuteResponse, RxchefClient } from './types';
import { OperationsPanel } from './components/OperationsPanel';
import { RecipePanel } from './components/RecipePanel';
import { InputPanel } from './components/InputPanel';
import { OutputPanel } from './components/OutputPanel';
import { TracePanel } from './components/TracePanel';
import { Sun, Moon, Shield, Key } from 'lucide-react';
import './styles/workbench.css';
import { encodeBase64, parseInput } from './lib/bytes';
import { prepareRecipe } from './lib/recipe';
import { historyReducer } from './lib/history';
import { CommandPalette } from './components/CommandPalette';
import { RecipeTools } from './components/RecipeTools';
const GraphEditor = React.lazy(() => import('./features/graph/GraphEditor').then(module => ({ default: module.GraphEditor })));
import { errorMessage } from './lib/contracts';

export const App: React.FC = () => {
  const [mode, setMode] = useState<'list' | 'graph'>('list');
  const [graphOpened, setGraphOpened] = useState(false);
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [token, setToken] = useState<string>('');
  const [connectionVersion, setConnectionVersion] = useState(0);
  const [remoteUrl, setRemoteUrl] = useState('');
  const [connectedRemote, setConnectedRemote] = useState(false);
  const [tokenDraft, setTokenDraft] = useState('');
  useEffect(() => {
    try { localStorage.removeItem('rxchef_token'); sessionStorage.removeItem('rxchef_token'); } catch { /* storage may be unavailable */ }
  }, []);
  const [showTokenDialog, setShowTokenDialog] = useState<boolean>(false);

  // Initialize client (Tauri if in desktop, else HTTP)
  const isTauri = typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);
  const client: RxchefClient = useMemo(() => {
    if (isTauri) {
      return new TauriRxchefClient();
    }
    return new HttpRxchefClient('', token || null);
  }, [isTauri, token, connectionVersion]);

  const [operations, setOperations] = useState<OperationDescriptor[]>([]);
  const [history, dispatch] = useReducer(historyReducer<RecipeStep[]>, { past: [], present: [{ op: 'To Base64', args: [] }], future: [] });
  const recipe = history.present;
  const setRecipe = (value: RecipeStep[] | ((steps: RecipeStep[]) => RecipeStep[])) => dispatch({ type: 'set', value });
  const [input, setInput] = useState<string>('Hello CyberChef from rx-chef!');
  const [inputType, setInputType] = useState<'text' | 'hex' | 'base64'>('text');
  const [outputViewMode, setOutputViewMode] = useState<'text' | 'hex' | 'base64'>('text');
  const [response, setResponse] = useState<ExecuteResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isExecuting, setIsExecuting] = useState<boolean>(false);
  const [palette, setPalette] = useState(false);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [favorites, setFavorites] = useState<string[]>(() => {
    try {
      return JSON.parse(localStorage.getItem('rxchef_favs') || '[]');
    } catch {
      return ['To Base64', 'From Base64', 'To Hex', 'From Hex', 'MD5', 'SHA256'];
    }
  });

  // Toggle Theme
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  // Save favorites
  useEffect(() => {
    try { localStorage.setItem('rxchef_favs', JSON.stringify(favorites)); } catch { /* private storage */ }
  }, [favorites]);

  // Load operations on start
  useEffect(() => {
    let active = true;
    setOperations([]);
    client
      .operations()
      .then((ops) => {
        if (!active) return;
        setOperations(ops);
        setError(null);
      })
      .catch((err) => {
        if (!active) return;
        setError(err.message || 'Failed to load operations from server');
        if (err.kind === 'unauthorized') {
          setShowTokenDialog(true);
        }
      });
    return () => { active = false; };
  }, [client]);

  const operationsMap = useMemo(() => {
    const map = new Map<string, OperationDescriptor>();
    operations.forEach((op) => map.set(op.name, op));
    return map;
  }, [operations]);

  // Execute Recipe
  const handleBake = useCallback(async () => {
    if (isExecuting) return;
    setIsExecuting(true);
    setError(null);

    let sourceIndices: number[] = [];
    try {
      const input_base64 = encodeBase64(parseInput(input, inputType));
      const prepared = prepareRecipe(recipe);
      const steps = prepared.steps; sourceIndices = prepared.sourceIndices;
      const request = { input_base64, recipe: steps, trace: {
        max_entries: 64, max_bytes_per_entry: 512, max_total_bytes: 65536,
      }};
      await client.validate(request);
      const res = await client.execute(request);
      res.trace = res.trace.map(entry => ({ ...entry, step_index: sourceIndices[entry.step_index] }));
      setResponse(res);
    } catch (err: unknown) {
      const step = err && typeof err === 'object' && 'step' in err && typeof err.step === 'number' ? sourceIndices[err.step - 1] : undefined;
      setError(`${step === undefined ? '' : `Step ${step + 1}: `}${errorMessage(err)}`);
      setResponse(null);
    } finally {
      setIsExecuting(false);
    }
  }, [client, input, inputType, recipe, isExecuting]);

  useEffect(() => {
    const shortcut = (event: KeyboardEvent) => {
      if (mode !== 'list' || !(event.metaKey || event.ctrlKey)) return;
      if (event.key === 'Enter') { event.preventDefault(); void handleBake(); }
      if (event.key.toLowerCase() === 'k') { event.preventDefault(); setPalette(value => !value); }
      if (event.key.toLowerCase() === 'z' && !(event.target instanceof HTMLElement && (event.target.closest('.cm-editor') || ['INPUT', 'TEXTAREA'].includes(event.target.tagName)))) {
        event.preventDefault(); dispatch({ type: event.shiftKey ? 'redo' : 'undo' });
      }
    };
    window.addEventListener('keydown', shortcut);
    return () => window.removeEventListener('keydown', shortcut);
  }, [handleBake, mode]);

  const handleAddOperation = (op: OperationDescriptor) => {
    const defaultArgs = op.args.map((a) => a.default || '');
    setRecipe((prev) => [...prev, { op: op.name, args: defaultArgs }]);
  };

  const handleRemoveStep = (idx: number) => {
    setRecipe((prev) => prev.filter((_, i) => i !== idx));
  };

  const handleMoveStep = (idx: number, dir: 'up' | 'down') => {
    setRecipe((prev) => {
      const copy = [...prev];
      const target = dir === 'up' ? idx - 1 : idx + 1;
      if (target < 0 || target >= copy.length) return prev;
      const temp = copy[idx];
      copy[idx] = copy[target];
      copy[target] = temp;
      return copy;
    });
  };

  const handleToggleDisabled = (idx: number) => {
    setRecipe((prev) =>
      prev.map((s, i) => (i === idx ? { ...s, disabled: !s.disabled } : s))
    );
  };

  const handleUpdateArg = (stepIdx: number, argIdx: number, val: string) => {
    setRecipe((prev) =>
      prev.map((s, i) => {
        if (i !== stepIdx) return s;
        const newArgs = [...s.args];
        newArgs[argIdx] = val;
        return { ...s, args: newArgs };
      })
    );
  };

  const handleToggleFavorite = (id: string) => {
    setFavorites((prev) =>
      prev.includes(id) ? prev.filter((f) => f !== id) : [...prev, id]
    );
  };

  const handleFileUpload = async (file: File) => {
    try {
      const capabilities = await client.capabilities();
      if (file.size > capabilities.max_input_bytes) throw new Error('File exceeds the engine input limit.');
      setInput(encodeBase64(new Uint8Array(await file.arrayBuffer())));
      setInputType('base64');
    } catch (error) { setError(errorMessage(error)); }
  };

  return (
    <>
      <header className="workbench-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Shield size={18} style={{ color: 'var(--accent-primary)' }} />
          <span style={{ fontWeight: 700, fontSize: '15px', letterSpacing: '-0.02em' }}>
            rx-chef
          </span>
          <span style={{ fontSize: '11px', color: 'var(--text-muted)', marginLeft: '4px' }}>
            v0.1.0 {isTauri ? (connectedRemote ? '(Desktop / Remote)' : '(Desktop / Local)') : '(Web / Remote)'}
          </span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          {(
            <button
              className="btn btn-secondary"
              onClick={() => { setTokenDraft(token); setShowTokenDialog(true); }}
              title="Server Auth Token"
            >
              <Key size={13} /> {isTauri ? 'Connection' : operations.length ? 'Connected' : 'Set Token'}
            </button>
          )}

          <button
            className="btn-icon"
            onClick={() => setTheme((t) => (t === 'dark' ? 'light' : 'dark'))}
            title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
          >
            {theme === 'dark' ? <Sun size={15} /> : <Moon size={15} />}
          </button>
        </div>
      </header>

      {mode === 'list' && palette && <CommandPalette operations={operations} onSelect={handleAddOperation} onClose={() => setPalette(false)} />}
      {mode === 'list' && <button className="palette-toggle btn btn-secondary" onClick={() => setPalette(true)}>Commands · ⌘/Ctrl K</button>}
      <nav className="workbench-mode" aria-label="Workbench mode"><button onClick={() => setMode('list')} aria-pressed={mode === 'list'}>List mode</button><button onClick={() => { setGraphOpened(true); setMode('graph'); }} aria-pressed={mode === 'graph'}>Graph mode</button></nav>
      <div hidden={mode !== 'graph'}>{graphOpened && <React.Suspense fallback={<p role="status">Loading graph editor…</p>}><GraphEditor client={client} operations={operations} theme={theme} visible={mode === 'graph'} /></React.Suspense>}</div>
      {mode === 'list' && <><RecipeTools recipe={recipe} onLoad={setRecipe} onError={setError} undo={() => dispatch({ type: 'undo' })} redo={() => dispatch({ type: 'redo' })} canUndo={!!history.past.length} canRedo={!!history.future.length} />
      {/* Main workbench */}
      <main className="workbench-main">
        <OperationsPanel
          operations={operations}
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          selectedCategory={selectedCategory}
          onSelectCategory={setSelectedCategory}
          onAddOperation={handleAddOperation}
          favorites={favorites}
          onToggleFavorite={handleToggleFavorite}
        />

        <RecipePanel
          recipe={recipe}
          operationsMap={operationsMap}
          onRemoveStep={handleRemoveStep}
          onMoveStep={handleMoveStep}
          onToggleDisabled={handleToggleDisabled}
          onUpdateArg={handleUpdateArg}
          onBake={handleBake}
          onClear={() => setRecipe([])}
          isExecuting={isExecuting}
        />

        <InputPanel
          input={input}
          onChangeInput={setInput}
          inputType={inputType}
          onChangeInputType={setInputType}
          onFileUpload={handleFileUpload}
          onNativeImport={isTauri ? async () => { try { const data = await importNativeFile(); if (data !== null) { setInput(data); setInputType('base64'); } } catch (error) { setError(errorMessage(error)); } } : undefined}
        />

        <OutputPanel
          response={response}
          error={error}
          outputViewMode={outputViewMode}
          onChangeViewMode={setOutputViewMode}
        />
      </main>

      {/* Trace inspector drawer */}
      {response && response.trace && (
        <TracePanel trace={response.trace} truncated={response.trace_truncated} />
      )}

      </>}
      {/* Token Modal */}
      {showTokenDialog && (
        <div
          style={{
            position: 'fixed',
            inset: 0,
            backgroundColor: 'rgba(0, 0, 0, 0.6)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            zIndex: 1000,
          }}
        >
          <div
            style={{
              backgroundColor: 'var(--bg-secondary)',
              border: '1px solid var(--border-strong)',
              borderRadius: 'var(--radius-lg)',
              padding: '20px',
              width: '380px',
              display: 'flex',
              flexDirection: 'column',
              gap: '12px',
              boxShadow: 'var(--shadow-md)',
            }}
          >
            <div style={{ fontWeight: 600, fontSize: '14px' }}>Server Bearer Token</div>
            <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>
              Enter the access token printed by <code>rxchef-server</code> on startup.
            </div>
            {isTauri && <label>Server URL (empty = local engine)<input aria-label="Server URL" type="text" value={remoteUrl} onChange={e => setRemoteUrl(e.target.value)} placeholder="https://chef.example" /></label>}
            <input
              type="password"
              placeholder="e.g. test-token-0123456789abcdef"
              value={tokenDraft}
              onChange={(e) => setTokenDraft(e.target.value)}
              autoComplete="off"
              aria-label="Server Bearer Token"
              style={{ width: '100%' }}
            />
            <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '8px', marginTop: '8px' }}>
              <button
                className="btn btn-secondary"
                onClick={() => setShowTokenDialog(false)}
              >
                Cancel
              </button>
              <button
                className="btn btn-primary"
                onClick={async () => {
                  if (isTauri) {
                    try { await configureConnection(remoteUrl, tokenDraft); setConnectedRemote(!!remoteUrl); setConnectionVersion(v => v + 1); }
                    catch (error) { setError(errorMessage(error)); return; }
                  }
                  setToken(isTauri ? '' : tokenDraft);
                  setTokenDraft('');
                  setShowTokenDialog(false);

                }}
              >
                Connect (this session)
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
