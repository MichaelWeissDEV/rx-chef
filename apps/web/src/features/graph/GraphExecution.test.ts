import { expect, it, vi } from 'vitest';
import { GraphExecution } from './GraphExecution';
import { starterGraph } from './model';
import type { GraphRequest, GraphResult, JobSnapshot, RxchefClient } from '../../types';
import fixture from '../../generated/contracts.fixture.json';
function deferred<T>() { let resolve!: (v: T) => void; const promise = new Promise<T>(r => { resolve = r; }); return { promise, resolve }; }
const response = () => structuredClone(fixture.graph_response) as GraphResult;
const request = (): GraphRequest => ({ graph: starterGraph(), inputs_base64: { input: 'AP/A' }, outputs: [] });
const flush = async () => { for (let i = 0; i < 10; i++) await Promise.resolve(); };
function local(execute: (r: GraphRequest) => Promise<GraphResult>) {
  return { capabilities: async () => ({ ...fixture.capabilities, jobs: false }), validateGraph: async () => {}, executeGraph: execute } as unknown as RxchefClient;
}
it('captures immutable graph and input values before awaiting transport', async () => {
  const gate = deferred<GraphResult>(); const execute = vi.fn((request: GraphRequest) => { void request; return gate.promise; });
  const run = new GraphExecution(); run.connect(local(execute)); const source = request(); const pending = run.start(source);
  source.graph.id = 'changed'; source.inputs_base64!.input = '';
  await flush(); expect(execute.mock.calls[0][0].graph.id).toBe('untitled');
  expect(execute.mock.calls[0][0].inputs_base64!.input).toBe('AP/A');
  expect(Object.isFrozen(run.state.snapshot!.request.graph.nodes)).toBe(true);
  gate.resolve(response()); await pending; expect(run.state.status).toBe('succeeded');
});
for (const edit of ['node deletion', 'port changes', 'undo/redo', 'input replacement']) {
  it(`discards results after ${edit}`, async () => {
    const gate = deferred<GraphResult>(); const run = new GraphExecution(); run.connect(local(() => gate.promise));
    const pending = run.start(request()); await flush();
    if (edit === 'input replacement') run.editInputs(); else run.editGraph();
    gate.resolve(response()); await pending;
    expect(run.state.status).toBe('discarded'); expect(run.state.result).toBeNull();
  });
}
it('accepts results across a presentation mode switch without changing graph state', async () => {
  const gate = deferred<GraphResult>(); const run = new GraphExecution(); run.connect(local(() => gate.promise));
  const pending = run.start(request()); gate.resolve(response()); await pending;
  expect(run.state.status).toBe('succeeded');
});
it('connection changes reject old results and preserve the new connection state', async () => {
  const gate = deferred<GraphResult>(); const run = new GraphExecution(); run.connect(local(() => gate.promise));
  const pending = run.start(request()); await flush(); run.connect(local(async () => response()));
  await run.start(request()); const current = run.state.snapshot!.run_id;
  gate.resolve(response()); await pending;
  expect(run.state.status).toBe('succeeded'); expect(run.state.snapshot!.run_id).toBe(current);
});
it('rapid repeated starts publish only the newest run', async () => {
  const a = deferred<GraphResult>(), b = deferred<GraphResult>(); let calls = 0;
  const run = new GraphExecution(); run.connect(local(() => ++calls === 1 ? a.promise : b.promise));
  const first = run.start(request()); await flush(); const second = run.start(request()); await flush();
  b.resolve(response()); await second; const id = run.state.snapshot!.run_id;
  a.resolve(response()); await first; expect(run.state.snapshot!.run_id).toBe(id); expect(id).toBe('graph-run-2');
});
it('cancels a job whose ID arrives after closing the connection', async () => {
  const submitted = deferred<JobSnapshot>(); const cancelJob = vi.fn(async () => ({ id: 'late', status: 'cancelled' }));
  const client = { capabilities: async () => ({ ...fixture.capabilities, jobs: true }), validateGraph: async () => {}, submitJob: () => submitted.promise, cancelJob } as unknown as RxchefClient;
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush(); run.close();
  submitted.resolve({ id: 'late', status: 'queued' }); await pending;
  expect(cancelJob).toHaveBeenCalledWith('late'); expect(run.state.result).toBeNull();
});
