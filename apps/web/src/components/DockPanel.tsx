import { useEffect, useRef, useState, type ReactNode } from 'react';
import { createPortal } from 'react-dom';

/** A local floating panel keeps the current bytes in this app and never opens a remote window. */
export function DockPanel({ title, children, className = '' }: { title: string; children: ReactNode; className?: string }) {
  const [collapsed, setCollapsed] = useState(false);
  const [floating, setFloating] = useState(false);
  const [position, setPosition] = useState<{ x: number; y: number } | null>(null);
  const stopDrag = useRef<(() => void) | null>(null);
  useEffect(() => () => stopDrag.current?.(), []);
  const panel = <section className={`dock-panel ${className} ${collapsed ? 'is-collapsed' : ''} ${floating ? 'is-floating' : ''}`} aria-label={title}
    role={floating ? 'dialog' : 'region'} style={floating && position ? { left: position.x, top: position.y, right: 'auto', bottom: 'auto' } : undefined}>
    <header className="dock-heading" onPointerDown={event => {
      if (!floating || (event.target as HTMLElement).closest('button')) return;
      stopDrag.current?.();
      const box = event.currentTarget.parentElement!.getBoundingClientRect();
      const start = { x: event.clientX, y: event.clientY };
      const move = (next: PointerEvent) => setPosition({ x: Math.max(0, Math.min(Math.max(0, innerWidth - box.width), box.x + next.clientX - start.x)), y: Math.max(0, Math.min(Math.max(0, innerHeight - box.height), box.y + next.clientY - start.y)) });
      const stop = () => { window.removeEventListener('pointermove', move); window.removeEventListener('pointerup', stop); };
      stopDrag.current = stop;
      window.addEventListener('pointermove', move); window.addEventListener('pointerup', stop, { once: true });
    }}><strong>{title}</strong><div>
      <button aria-label={`${collapsed ? 'Expand' : 'Collapse'} ${title}`} aria-expanded={!collapsed} onClick={() => setCollapsed(value => !value)}>{collapsed ? 'Expand' : 'Collapse'}</button>
      <button aria-label={`${floating ? 'Dock' : 'Undock'} ${title}`} onClick={() => { setFloating(value => !value); setCollapsed(false); setPosition(null); }}>{floating ? 'Dock' : 'Undock'}</button>
    </div></header>
    <div className="dock-body" hidden={collapsed}>{children}</div>
  </section>;
  return <>{floating ? <><button className="dock-restore" onClick={() => { setFloating(false); setCollapsed(false); }}>Dock {title}</button>{createPortal(panel, document.body)}</> : panel}</>;
}
