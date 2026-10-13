import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { HttpRxchefClient } from './clients/http';
import { TauriRxchefClient } from './clients/tauri';
import { OperationDescriptor, RecipeStep, ExecuteResponse, RxchefClient } from './types';
import { OperationsPanel } from './components/OperationsPanel';
import { RecipePanel } from './components/RecipePanel';
import { InputPanel } from './components/InputPanel';
import { OutputPanel } from './components/OutputPanel';
import { TracePanel } from './components/TracePanel';
import { Sun, Moon, Shield, Key } from 'lucide-react';
import './styles/workbench.css';

export const App: React.FC = () => {
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [token, setToken] = useState<string>(() => localStorage.getItem('rxchef_token') || '');
  const [showTokenDialog, setShowTokenDialog] = useState<boolean>(false);

  // Initialize client (Tauri if in desktop, else HTTP)
  const isTauri = typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);
  const client: RxchefClient = useMemo(() => {
    if (isTauri) {
      return new TauriRxchefClient();
    }
    return new HttpRxchefClient('', token || null);
  }, [isTauri, token]);

  const [operations, setOperations] = useState<OperationDescriptor[]>([]);
  const [recipe, setRecipe] = useState<RecipeStep[]>([
    { op: 'To Base64', args: [] },
  ]);
  const [input, setInput] = useState<string>('Hello CyberChef from rx-chef!');
  const [inputType, setInputType] = useState<'text' | 'hex' | 'base64'>('text');
  const [outputViewMode, setOutputViewMode] = useState<'text' | 'hex' | 'base64'>('text');
  const [response, setResponse] = useState<ExecuteResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isExecuting, setIsExecuting] = useState<boolean>(false);
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
    localStorage.setItem('rxchef_favs', JSON.stringify(favorites));
  }, [favorites]);

  // Load operations on start
  useEffect(() => {
    client
      .operations()
      .then((ops) => {
        setOperations(ops);
        setError(null);
      })
      .catch((err) => {
        setError(err.message || 'Failed to load operations from server');
        if (err.kind === 'unauthorized') {
          setShowTokenDialog(true);
        }
      });
  }, [client]);

  const operationsMap = useMemo(() => {
    const map = new Map<string, OperationDescriptor>();
    operations.forEach((op) => map.set(op.name, op));
    return map;
  }, [operations]);

  // Execute Recipe
  const handleBake = useCallback(async () => {
    setIsExecuting(true);
    setError(null);

    try {
      // Encode input to Base64
      let inputBytes: Uint8Array;
      if (inputType === 'hex') {
        const cleanHex = input.replace(/\s+/g, '');
        const matches = cleanHex.match(/.{1,2}/g) || [];
        inputBytes = new Uint8Array(matches.map((byte) => parseInt(byte, 16)));
      } else if (inputType === 'base64') {
        const binary = atob(input);
        inputBytes = new Uint8Array(binary.length);
        for (let i = 0; i < binary.length; i++) {
          inputBytes[i] = binary.charCodeAt(i);
        }
      } else {
        const encoder = new TextEncoder();
        inputBytes = encoder.encode(input);
      }

      // Convert Uint8Array to Base64 string safely
      let binaryStr = '';
      for (let i = 0; i < inputBytes.length; i++) {
        binaryStr += String.fromCharCode(inputBytes[i]);
      }
      const input_base64 = btoa(binaryStr);

      const activeRecipe = recipe.filter((s) => !s.disabled);
      const res = await client.execute({
        input_base64,
        recipe: activeRecipe,
        trace: {
          max_entries: 64,
          max_bytes_per_entry: 512,
          max_total_bytes: 65536,
        },
      });

      setResponse(res);
    } catch (err: any) {
      setError(err.message || String(err));
      setResponse(null);
    } finally {
      setIsExecuting(false);
    }
  }, [client, input, inputType, recipe]);

  const handleAddOperation = (op: OperationDescriptor) => {
    const defaultArgs = op.args.map((a) => a.default_value || '');
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

  const handleFileUpload = (file: File) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (reader.result instanceof ArrayBuffer) {
        const bytes = new Uint8Array(reader.result);
        let binary = '';
        for (let i = 0; i < bytes.length; i++) {
          binary += String.fromCharCode(bytes[i]);
        }
        setInput(btoa(binary));
        setInputType('base64');
      }
    };
    reader.readAsArrayBuffer(file);
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
            v0.1.0 {isTauri ? '(Desktop Standalone)' : '(Web / Remote)'}
          </span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          {!isTauri && (
            <button
              className="btn btn-secondary"
              onClick={() => setShowTokenDialog(true)}
              title="Server Auth Token"
            >
              <Key size={13} /> {token ? 'Authenticated' : 'Set Token'}
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

      {/* Main 4-column workbench */}
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
            <input
              type="password"
              placeholder="e.g. test-token-0123456789abcdef"
              value={token}
              onChange={(e) => setToken(e.target.value)}
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
                onClick={() => {
                  localStorage.setItem('rxchef_token', token);
                  setShowTokenDialog(false);
                  window.location.reload();
                }}
              >
                Save & Connect
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
