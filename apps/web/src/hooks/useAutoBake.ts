import { useEffect, useRef } from 'react';

/** Debounce edits, submit each revision once, and wait for a previous worker to finish. */
export function useAutoBake({ enabled, visible, ready, running, revision, run }: { enabled: boolean; visible: boolean; ready: boolean; running: boolean; revision: string; run(): void | Promise<void> }) {
  const attempted = useRef<string | null>(null);
  const latestRun = useRef(run); latestRun.current = run;
  useEffect(() => {
    if (!enabled) { attempted.current = null; return; }
    if (!visible || !ready || running || attempted.current === revision) return;
    const timer = setTimeout(() => { attempted.current = revision; void latestRun.current(); }, 500);
    return () => clearTimeout(timer);
  }, [enabled, visible, ready, running, revision]);
}
