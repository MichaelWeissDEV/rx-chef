import React, { useMemo } from 'react';
import { OperationDescriptor } from '../types';
import { Search, Plus, Star } from 'lucide-react';

interface OperationsPanelProps {
  operations: OperationDescriptor[];
  searchQuery: string;
  onSearchChange: (query: string) => void;
  selectedCategory: string;
  onSelectCategory: (category: string) => void;
  onAddOperation: (op: OperationDescriptor) => void;
  favorites: string[];
  onToggleFavorite: (id: string) => void;
}

export const OperationsPanel: React.FC<OperationsPanelProps> = ({
  operations,
  searchQuery,
  onSearchChange,
  selectedCategory,
  onSelectCategory,
  onAddOperation,
  favorites,
  onToggleFavorite,
}) => {
  const categories = useMemo(() => {
    const set = new Set<string>();
    operations.forEach((op) => {
      if (op.category) set.add(op.category);
    });
    return Array.from(set).sort();
  }, [operations]);

  const filteredOperations = useMemo(() => {
    return operations.filter((op) => {
      const matchesSearch =
        searchQuery === '' ||
        op.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        op.description.toLowerCase().includes(searchQuery.toLowerCase());

      const matchesCat =
        selectedCategory === 'All' ||
        (selectedCategory === 'Favorites' && favorites.includes(op.id)) ||
        op.category === selectedCategory;

      return matchesSearch && matchesCat;
    });
  }, [operations, searchQuery, selectedCategory, favorites]);

  return (
    <div className="panel" style={{ width: '280px', minWidth: '240px' }}>
      <div className="panel-header">
        <span>Operations ({filteredOperations.length})</span>
      </div>

      <div style={{ padding: '8px', borderBottom: '1px solid var(--border-subtle)' }}>
        <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
          <Search size={14} style={{ position: 'absolute', left: '8px', color: 'var(--text-muted)' }} />
          <input
            type="search"
            placeholder="Search operations..."
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            style={{ width: '100%', paddingLeft: '28px' }}
          />
        </div>

        <div style={{ display: 'flex', gap: '4px', overflowX: 'auto', marginTop: '8px', paddingBottom: '2px' }}>
          <button
            className={`btn ${selectedCategory === 'All' ? 'btn-primary' : 'btn-secondary'}`}
            style={{ padding: '2px 8px', fontSize: '11px' }}
            onClick={() => onSelectCategory('All')}
          >
            All
          </button>
          <button
            className={`btn ${selectedCategory === 'Favorites' ? 'btn-primary' : 'btn-secondary'}`}
            style={{ padding: '2px 8px', fontSize: '11px' }}
            onClick={() => onSelectCategory('Favorites')}
          >
            ★ Favs
          </button>
          {categories.map((cat) => (
            <button
              key={cat}
              className={`btn ${selectedCategory === cat ? 'btn-primary' : 'btn-secondary'}`}
              style={{ padding: '2px 8px', fontSize: '11px', whiteSpace: 'nowrap' }}
              onClick={() => onSelectCategory(cat)}
            >
              {cat}
            </button>
          ))}
        </div>
      </div>

      <div className="panel-body">
        {filteredOperations.map((op) => {
          const isFav = favorites.includes(op.id);
          return (
            <div
              key={op.id}
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '6px 10px',
                borderBottom: '1px solid var(--border-subtle)',
                backgroundColor: 'transparent',
                cursor: 'pointer',
              }}
              className="op-item"
              onDoubleClick={() => onAddOperation(op)}
            >
              <div style={{ overflow: 'hidden', paddingRight: '8px', flex: 1 }}>
                <div style={{ fontWeight: 500, fontSize: '12px', whiteSpace: 'nowrap', textOverflow: 'ellipsis', overflow: 'hidden' }}>
                  {op.name}
                </div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', whiteSpace: 'nowrap', textOverflow: 'ellipsis', overflow: 'hidden' }}>
                  {op.category}
                </div>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                <button
                  className="btn-icon"
                  style={{ color: isFav ? 'var(--accent-primary)' : 'var(--text-muted)' }}
                  onClick={(e) => {
                    e.stopPropagation();
                    onToggleFavorite(op.id);
                  }}
                  title={isFav ? 'Remove from favorites' : 'Add to favorites'}
                >
                  <Star size={13} fill={isFav ? 'currentColor' : 'none'} />
                </button>
                <button
                  className="btn-icon"
                  onClick={() => onAddOperation(op)}
                  title="Add to Recipe"
                >
                  <Plus size={14} />
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
