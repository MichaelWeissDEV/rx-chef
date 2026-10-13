import React from 'react';
import { ArgSchema, OperationDescriptor, RecipeStep } from '../types';
import { Trash2, ChevronUp, ChevronDown, Eye, EyeOff, Play } from 'lucide-react';

interface RecipePanelProps {
  recipe: RecipeStep[];
  operationsMap: Map<string, OperationDescriptor>;
  onRemoveStep: (index: number) => void;
  onMoveStep: (index: number, direction: 'up' | 'down') => void;
  onToggleDisabled: (index: number) => void;
  onUpdateArg: (stepIndex: number, argIndex: number, value: string) => void;
  onBake: () => void;
  onClear: () => void;
  isExecuting: boolean;
}

export const RecipePanel: React.FC<RecipePanelProps> = ({
  recipe,
  operationsMap,
  onRemoveStep,
  onMoveStep,
  onToggleDisabled,
  onUpdateArg,
  onBake,
  onClear,
  isExecuting,
}) => {
  return (
    <div className="panel" style={{ width: '380px', minWidth: '320px' }}>
      <div className="panel-header">
        <span>Recipe ({recipe.length} steps)</span>
        <div style={{ display: 'flex', gap: '4px' }}>
          <button className="btn btn-secondary" style={{ padding: '2px 8px' }} onClick={onClear} title="Clear recipe">
            Clear
          </button>
          <button className="btn btn-primary" style={{ padding: '2px 10px' }} onClick={onBake} disabled={isExecuting}>
            <Play size={12} fill="currentColor" /> {isExecuting ? 'Baking...' : 'Bake'}
          </button>
        </div>
      </div>

      <div className="panel-body" style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {recipe.length === 0 ? (
          <div style={{ padding: '32px 16px', textAlign: 'center', color: 'var(--text-muted)' }}>
            Double-click or click + on operations to build your recipe.
          </div>
        ) : (
          recipe.map((step, stepIdx) => {
            const desc = operationsMap.get(step.op);
            const isControlFlow = ['Fork', 'Merge', 'Subsection', 'Register', 'Jump', 'Return'].includes(step.op);

            return (
              <div
                key={stepIdx}
                style={{
                  backgroundColor: step.disabled ? 'var(--bg-primary)' : 'var(--bg-tertiary)',
                  border: `1px solid ${isControlFlow ? 'var(--accent-primary)' : 'var(--border-strong)'}`,
                  borderRadius: 'var(--radius-sm)',
                  opacity: step.disabled ? 0.6 : 1,
                  overflow: 'hidden',
                }}
              >
                {/* Step Header */}
                <div
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '6px 8px',
                    backgroundColor: 'var(--bg-surface)',
                    borderBottom: '1px solid var(--border-subtle)',
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontWeight: 600 }}>
                    <span style={{ color: 'var(--text-muted)', fontSize: '11px' }}>{stepIdx + 1}.</span>
                    <span>{step.op}</span>
                  </div>

                  <div style={{ display: 'flex', alignItems: 'center', gap: '2px' }}>
                    <button
                      className="btn-icon"
                      onClick={() => onMoveStep(stepIdx, 'up')}
                      disabled={stepIdx === 0}
                      title="Move Up"
                    >
                      <ChevronUp size={13} />
                    </button>
                    <button
                      className="btn-icon"
                      onClick={() => onMoveStep(stepIdx, 'down')}
                      disabled={stepIdx === recipe.length - 1}
                      title="Move Down"
                    >
                      <ChevronDown size={13} />
                    </button>
                    <button
                      className="btn-icon"
                      onClick={() => onToggleDisabled(stepIdx)}
                      title={step.disabled ? 'Enable step' : 'Disable step'}
                    >
                      {step.disabled ? <EyeOff size={13} /> : <Eye size={13} />}
                    </button>
                    <button
                      className="btn-icon"
                      onClick={() => onRemoveStep(stepIdx)}
                      title="Remove step"
                    >
                      <Trash2 size={13} />
                    </button>
                  </div>
                </div>

                {/* Step Arguments */}
                {desc && desc.args && desc.args.length > 0 && (
                  <div style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                    {desc.args.map((argSchema: ArgSchema, argIdx: number) => {
                      const val = step.args[argIdx] ?? argSchema.default_value ?? '';

                      return (
                        <div key={argIdx} style={{ display: 'flex', flexDirection: 'column', gap: '2px' }}>
                          <label style={{ fontSize: '11px', color: 'var(--text-secondary)' }}>
                            {argSchema.name}
                          </label>
                          {argSchema.choices && argSchema.choices.length > 0 ? (
                            <select
                              value={val}
                              onChange={(e) => onUpdateArg(stepIdx, argIdx, e.target.value)}
                              style={{ width: '100%' }}
                            >
                              {argSchema.choices.map((c: string) => (
                                <option key={c} value={c}>
                                  {c}
                                </option>
                              ))}
                            </select>
                          ) : (
                            <input
                              type={argSchema.sensitive ? 'password' : 'text'}
                              value={val}
                              onChange={(e) => onUpdateArg(stepIdx, argIdx, e.target.value)}
                              placeholder={argSchema.default_value}
                              style={{ width: '100%' }}
                            />
                          )}
                        </div>
                      );
                    })}
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};
