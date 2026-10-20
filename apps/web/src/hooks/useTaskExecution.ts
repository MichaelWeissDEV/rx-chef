import { useEffect, useRef, useState } from 'react';
import type { EngineTask, RxchefClient } from '../types';
import { executeTask } from '../lib/jobs';
interface Active { client: RxchefClient; cancelled: boolean; id?: string }
export function useTaskExecution(client: RxchefClient) {
  const active = useRef<Active | null>(null);
  const [state, setState] = useState({ client, running: false, canCancel: false });
  useEffect(() => {
    setState({ client, running: false, canCancel: false });
    return () => {
      const run = active.current; active.current = null;
      if (run) { run.cancelled = true; if (run.id) void run.client.cancelJob(run.id).catch(() => {}); }
    };
  }, [client]);
  const cancel = async () => { const run = active.current; if (!run) return; run.cancelled = true; if (run.id) await run.client.cancelJob(run.id); };
  const run = async (task: EngineTask) => {
    const snapshot = structuredClone(task);
    const previous = active.current;
    if (previous) { previous.cancelled = true; if (previous.id) void previous.client.cancelJob(previous.id).catch(() => {}); }
    const execution: Active = { client, cancelled: false }; active.current = execution;
    setState({ client, running: true, canCancel: true });
    try {
      const caps = await client.capabilities();
      if (active.current !== execution) return;
      setState({ client, running: true, canCancel: caps.jobs });
      const result = await executeTask(client, snapshot, {
        current: () => active.current === execution,
        cancelled: () => execution.cancelled,
        job: id => { execution.id = id; }, status: () => {},
      }, caps);
      if (active.current !== execution) return;
      if (execution.cancelled) throw new Error('Execution completed before cancellation; result discarded.');
      return result;
    } catch (error) { if (active.current === execution) throw error; }
    finally { if (active.current === execution) { active.current = null; setState({ client, running: false, canCancel: false }); } }
  };
  return { running: state.client === client && state.running, canCancel: state.client === client && state.canCancel, run, cancel };
}
