import React, { useState, useEffect, useMemo, useCallback, useReducer, useRef } from 'react';
import { HttpRxchefClient } from './clients/http';
import { TauriRxchefClient, configureConnection, importNativeFile } from './clients/tauri';
import { OperationDescriptor, RecipeStep, ExecuteResponse, RxchefClient, Graph, WorkbenchProject, WorkbenchRun } from './types';
import { OperationsPanel } from './components/OperationsPanel';
import { RecipePanel } from './components/RecipePanel';
import { InputPanel } from './components/InputPanel';
import { RecipeResults } from './components/RecipeResults';
import { TracePanel } from './components/TracePanel';
import { Sun, Moon, Shield, Key } from 'lucide-react';
import './styles/workbench.css';
import { encodeBase64, parseInput } from './lib/bytes';
import { prepareRecipe } from './lib/recipe';
import { historyReducer } from './lib/history';
import { CommandPalette } from './components/CommandPalette';
import { RecipeTools } from './components/RecipeTools';
const GraphEditor = React.lazy(() => import('./features/graph/GraphEditor').then(module => ({ default: module.GraphEditor })));
import { useTaskExecution } from './hooks/useTaskExecution';
import { errorMessage } from './lib/contracts';
import { ProjectPanel } from './components/ProjectPanel';
import { readPreferenceList } from './lib/preferences';
import { DockPanel } from './components/DockPanel';
import { useAutoBake } from './hooks/useAutoBake';
import { starterGraph } from './features/graph/model';

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
  const [project, setProject] = useState<WorkbenchProject>(() => ({ version:1,name:'Workspace',graphs:[starterGraph()],recipes:[{name:'Recipe 1',steps:[]}],settings:{theme:'dark',selected_graph:'untitled',selected_recipe:'Recipe 1'},input_files:[],history:[] }));
  const [documentVersion, setDocumentVersion] = useState(0);
  const [projectEpoch,setProjectEpoch] = useState(0);
  const projectRef = useRef(project); projectRef.current = project;
  const currentProject = useMemo<WorkbenchProject>(() => ({ ...project, settings:{...project.settings,theme}, recipes:project.recipes?.map(saved => saved.name === project.settings?.selected_recipe ? {...saved,steps:recipe.map(step=>({op:step.op,args:step.args,enabled:!step.disabled,breakpoint:step.breakpoint ?? false}))} : saved) }), [project,recipe,theme]);
  const setRecipe = (value: RecipeStep[] | ((steps: RecipeStep[]) => RecipeStep[])) => dispatch({ type: 'set', value });
  const [input, setInput] = useState<string>('Hello CyberChef from rx-chef!');
  const [inputType, setInputType] = useState<'text' | 'hex' | 'base64'>('text');
  const [outputViewMode, setOutputViewMode] = useState<'text' | 'hex' | 'base64'>('text');
  const [response, setResponse] = useState<ExecuteResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const recipeExecution = useTaskExecution(client);
  const isExecuting = recipeExecution.running;
  const [autoBake, setAutoBake] = useState(false);
  const [listOperations, setListOperations] = useState(true);
  const [listInput, setListInput] = useState(true);
  const [traceOpen, setTraceOpen] = useState(false);
  const recipeRevision = JSON.stringify([input, inputType, recipe, connectionVersion, projectEpoch, token]);
  const latestRecipeRevision = useRef(recipeRevision); latestRecipeRevision.current = recipeRevision;
  const [palette, setPalette] = useState(false);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [favorites, setFavorites] = useState(() => readPreferenceList('rxchef_favs', ['to_base64', 'from_base64', 'to_hex', 'from_hex']));

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
    setError(null);
    const owner = latestRecipeRevision.current;

    let sourceIndices: number[] = [];
    try {
      const input_base64 = encodeBase64(parseInput(input, inputType));
      const prepared = prepareRecipe(recipe);
      const steps = prepared.steps; sourceIndices = prepared.sourceIndices;
      const request = { input_base64, recipe: steps, trace: {
        max_entries: 64, max_bytes_per_entry: 512, max_total_bytes: 65536,
      }};
      const outcome = await recipeExecution.run({ method: 'execute', params: request });
      if (!outcome || latestRecipeRevision.current !== owner) return;
      if (outcome.kind !== 'execution') throw new Error('Worker returned the wrong result.');
      const res = outcome.value;
      res.trace = res.trace.map(entry => ({ ...entry, step_index: sourceIndices[entry.step_index] }));
      setResponse(res);
    } catch (err: unknown) {
      if (latestRecipeRevision.current !== owner) return;
      const step = err && typeof err === 'object' && 'step' in err && typeof err.step === 'number' ? sourceIndices[err.step - 1] : undefined;
      setError(`${step === undefined ? '' : `Step ${step + 1}: `}${errorMessage(err)}`);
      setResponse(null);
    }
  }, [client, input, inputType, recipe, isExecuting, recipeExecution]);

  useEffect(() => {
    setResponse(null);
    void recipeExecution.cancel().catch(() => {});
  }, [recipeRevision]);
  useAutoBake({ enabled: autoBake, visible: mode === 'list', ready: operations.length > 0, running: isExecuting, revision: recipeRevision, run: handleBake });

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
  const graphChanged = useCallback((graph:Graph) => {
    const current = projectRef.current; const selected = current.settings?.selected_graph;
    if (current.graphs?.some(saved=>saved.id===graph.id && saved.id!==selected)) {setError('Graph name already exists in this project. Choose another name.');return;}
    setProject(previous=>{
      const index = previous.graphs?.findIndex(saved=>saved.id===previous.settings?.selected_graph) ?? -1;
      const before=index>=0?previous.graphs?.[index]:undefined;
      const inputNames=new Map(before?.nodes.flatMap(node=>{
        if(node.type!=='input')return [];const after=graph.nodes.find(updated=>updated.id===node.id&&updated.type==='input');
        return after?.type==='input'?[[node.name,after.name] as const]:[];
      })??[]);
      const graphs=[...(previous.graphs ?? [])]; if(index>=0) graphs[index]=graph;else graphs.push(graph);
      const input_files=previous.input_files?.flatMap(reference=>reference.graph_id!==before?.id?[reference]:inputNames.has(reference.input_name)?[{...reference,graph_id:graph.id,input_name:inputNames.get(reference.input_name)!}]:[]);
      return {...previous,graphs,input_files,settings:{...previous.settings,selected_graph:graph.id}};
    });
  }, []);
  const graphRun = useCallback((run:WorkbenchRun) => {setProject(previous=>({...previous,history:[...(previous.history??[]).slice(-999),run]}));}, []);
  const openProject = (next:WorkbenchProject) => {
    setProjectEpoch(value=>value+1);
    void recipeExecution.cancel().catch(error=>setError(errorMessage(error)));
    const graphs=next.graphs?.length ? next.graphs : [starterGraph()];
    const recipes=next.recipes?.length ? next.recipes : [{name:'Recipe 1',steps:[]}];
    const graphId=next.settings?.selected_graph ?? graphs[0].id; const recipeName=next.settings?.selected_recipe ?? recipes[0].name;
    setProject({...next,graphs,recipes,settings:{...next.settings,selected_graph:graphId,selected_recipe:recipeName}});
    dispatch({type:'reset',value:recipes.find(saved=>saved.name===recipeName)!.steps.map(step=>({op:step.op,args:step.args ?? [],disabled:step.enabled===false,breakpoint:step.breakpoint ?? false}))});
    setTheme(next.settings?.theme ?? 'dark');setInput('');setResponse(null);setError(null);setGraphOpened(true);setDocumentVersion(v=>v+1);
  };
  const selectProjectRecipe = (next:WorkbenchProject,name:string) => {
    void recipeExecution.cancel().catch(error=>setError(errorMessage(error)));
    const saved=next.recipes?.find(recipe=>recipe.name===name);if(!saved)return;
    setProject({...next,settings:{...next.settings,selected_recipe:name}});
    dispatch({type:'reset',value:saved.steps.map(step=>({op:step.op,args:step.args??[],disabled:step.enabled===false,breakpoint:step.breakpoint??false}))});
    setInput('');setResponse(null);setError(null);setMode('list');
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
            v0.0.1 {isTauri ? (connectedRemote ? '(Desktop / Remote)' : '(Desktop / Local)') : '(Web / Remote)'}
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
      <div className="workspace-bar"><ProjectPanel project={currentProject} client={client} onOpen={openProject}
        onRename={name=>setProject(previous=>({...previous,name}))}
        onSelectGraph={id=>{setProject({...currentProject,settings:{...currentProject.settings,selected_graph:id}});setDocumentVersion(v=>v+1);setGraphOpened(true);setMode('graph');}}
        onSelectRecipe={name=>selectProjectRecipe(currentProject,name)}
        onAddGraph={()=>{const graph={...starterGraph(),id:`graph-${crypto.randomUUID().slice(0,8)}`};setProject({...currentProject,graphs:[...(currentProject.graphs??[]),graph],settings:{...currentProject.settings,selected_graph:graph.id}});setDocumentVersion(v=>v+1);setGraphOpened(true);setMode('graph');}}
        onAddRecipe={()=>{const name=`Recipe ${crypto.randomUUID().slice(0,8)}`;selectProjectRecipe({...currentProject,recipes:[...(currentProject.recipes??[]),{name,steps:[]}]},name);}} />

      <nav className="workbench-mode" aria-label="Workbench mode"><button onClick={() => setMode('list')} aria-pressed={mode === 'list'}>List mode</button><button onClick={() => { setGraphOpened(true); setMode('graph'); }} aria-pressed={mode === 'graph'}>Graph mode</button></nav></div>
      {mode === 'list' && palette && <CommandPalette operations={operations} onSelect={handleAddOperation} onClose={() => setPalette(false)} />}
      <div className="graph-host" hidden={mode !== 'graph'}>{graphOpened && <React.Suspense fallback={<p role="status">Loading graph editor…</p>}><GraphEditor client={client} operations={operations} favorites={favorites} onToggleFavorite={handleToggleFavorite} theme={theme} visible={mode === 'graph'} reservedIds={project.graphs?.filter(graph => graph.id !== project.settings?.selected_graph).map(graph => graph.id)} document={project.graphs?.find(graph=>graph.id===project.settings?.selected_graph)} documentVersion={documentVersion} projectEpoch={projectEpoch} onDocumentChange={graphChanged} onRun={graphRun} /></React.Suspense>}</div>
      {mode === 'list' && <><div className="list-toolbar"><button onClick={() => setPalette(true)}>Commands · ⌘/Ctrl K</button><button aria-pressed={listOperations} onClick={() => setListOperations(value => !value)}>Operations sidebar</button><button aria-pressed={listInput} onClick={() => setListInput(value => !value)}>Input panel</button><label><input type="checkbox" checked={autoBake} onChange={event => setAutoBake(event.target.checked)} />Auto-Bake recipe</label><button aria-pressed={traceOpen} onClick={() => setTraceOpen(value => !value)}>Transformation trace</button><details className="recipe-file-menu"><summary>Recipe files & tools</summary><RecipeTools recipe={recipe} onLoad={setRecipe} onError={setError} undo={() => dispatch({ type: 'undo' })} redo={() => dispatch({ type: 'redo' })} canUndo={!!history.past.length} canRedo={!!history.future.length} /></details></div>
      {/* Main workbench */}
      <main className="workbench-main">
        <div className="list-operations-shell" hidden={!listOperations}><OperationsPanel
          operations={operations}
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          selectedCategory={selectedCategory}
          onSelectCategory={setSelectedCategory}
          onAddOperation={handleAddOperation}
          favorites={favorites}
          onToggleFavorite={handleToggleFavorite}
        /></div>

        <RecipePanel
          recipe={recipe}
          operationsMap={operationsMap}
          onRemoveStep={handleRemoveStep}
          onMoveStep={handleMoveStep}
          onToggleDisabled={handleToggleDisabled}
          onUpdateArg={handleUpdateArg}
          onBake={handleBake}
          onCancel={() => { void recipeExecution.cancel().catch(error => setError(errorMessage(error))); }}
          canCancel={recipeExecution.canCancel}
          onClear={() => setRecipe([])}
          isExecuting={isExecuting}
        />

        <div className="list-input-shell" hidden={!listInput}><InputPanel
          input={input}
          onChangeInput={setInput}
          inputType={inputType}
          onChangeInputType={setInputType}
          onFileUpload={handleFileUpload}
          onNativeImport={isTauri ? async () => { try { const data = await importNativeFile(); if (data !== null) { setInput(data); setInputType('base64'); } } catch (error) { setError(errorMessage(error)); } } : undefined}
        /></div>

        <DockPanel title="Recipe results"><RecipeResults
          response={response}
          error={error}
          outputViewMode={outputViewMode}
          onChangeViewMode={setOutputViewMode}
        /></DockPanel>
      </main>

      {/* Trace inspector drawer */}
      {traceOpen && response && response.trace && (
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
