import type { Capabilities, EngineTask, JobSnapshot, RxchefClient } from '../types';
export interface TaskControl {
  current(): boolean; cancelled(): boolean; job(id: string): void;
  status(status: 'queued' | 'running'): void;
  snapshot?(snapshot:JobSnapshot):void;
}
const interrupted = () => ({ kind: 'cancelled', message: 'Execution was superseded or cancelled before dispatch.' });
/** One transport lifecycle for graph and recipe work; no domain execution here. */
export async function executeTask(client: RxchefClient, task: EngineTask, control: TaskControl, capabilities: Capabilities): Promise<NonNullable<JobSnapshot['result']>> {
  if (!control.current() || control.cancelled()) throw interrupted();
  if (!capabilities.jobs) {
    control.status('running');
    if (task.method === 'execute') return { kind: 'execution', value: await client.execute(task.params) };
    if (task.method === 'graph_execute') return { kind: 'graph', value: await client.executeGraph(task.params) };
    throw new Error('This task requires supervised jobs.');
  }
  const submitted = await client.submitJob(task); control.job(submitted.id);
  if (!control.current() || control.cancelled()) await client.cancelJob(submitted.id);
  if (!control.current()) throw interrupted();
  for (;;) {
    const state = await client.getJob(submitted.id);
    if (!control.current()) throw interrupted();
    control.snapshot?.(state);
    if (state.status === 'succeeded') {
      if (!state.result) throw new Error('Job returned no result.');
      return state.result;
    }
    if (state.status === 'failed' || state.status === 'cancelled' || state.status === 'timed_out') throw state.error ?? { kind: state.status === 'timed_out' ? 'timeout' : state.status, message: `Execution ${state.status}` };
    control.status(state.status === 'running' ? 'running' : 'queued');
    await new Promise(resolve => setTimeout(resolve, 100));
  }
}
