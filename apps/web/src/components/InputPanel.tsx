import React, { useRef } from 'react';
import { Upload, Copy } from 'lucide-react';

interface InputPanelProps {
  input: string;
  onChangeInput: (val: string) => void;
  inputType: 'text' | 'hex' | 'base64';
  onChangeInputType: (type: 'text' | 'hex' | 'base64') => void;
  onFileUpload: (file: File) => void;
}

export const InputPanel: React.FC<InputPanelProps> = ({
  input,
  onChangeInput,
  inputType,
  onChangeInputType,
  onFileUpload,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      onFileUpload(e.dataTransfer.files[0]);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(input);
  };

  return (
    <div className="panel" style={{ flex: 1, minWidth: '300px' }} onDragOver={(e) => e.preventDefault()} onDrop={handleDrop}>
      <div className="panel-header">
        <span>Input ({input.length} chars)</span>
        <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
          <select
            value={inputType}
            onChange={(e) => onChangeInputType(e.target.value as any)}
            style={{ padding: '2px 6px', fontSize: '11px' }}
          >
            <option value="text">UTF-8 Text</option>
            <option value="hex">HEX bytes</option>
            <option value="base64">Base64</option>
          </select>
          <input
            type="file"
            ref={fileInputRef}
            style={{ display: 'none' }}
            onChange={(e) => {
              if (e.target.files && e.target.files.length > 0) {
                onFileUpload(e.target.files[0]);
              }
            }}
          />
          <button
            className="btn-icon"
            onClick={() => fileInputRef.current?.click()}
            title="Import file"
          >
            <Upload size={13} />
          </button>
          <button className="btn-icon" onClick={handleCopy} title="Copy to clipboard">
            <Copy size={13} />
          </button>
        </div>
      </div>

      <div className="panel-body">
        <textarea
          className="editor"
          value={input}
          onChange={(e) => onChangeInput(e.target.value)}
          placeholder="Input data goes here, or drag & drop a file..."
          spellCheck={false}
        />
      </div>
    </div>
  );
};
