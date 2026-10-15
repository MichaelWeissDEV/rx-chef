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

function supervised(overrides: Partial<RxchefClient> = {}) {
  return {
    capabilities: async () => ({ ...fixture.capabilities, jobs: true }),
    validateGraph: async () => {},
    submitJob: vi.fn(async () => ({ id: 'job', status: 'queued' })),
    cancelJob: vi.fn(async () => ({ id: 'job', status: 'running' })),
    getJob: vi.fn(async () => ({ id: 'job', status: 'succeeded', result: { kind: 'graph', value: response() } })),
    ...overrides,
  } as unknown as RxchefClient;
}
for (const edit of ['graph', 'inputs']) {
  it(`remembers ${edit} edits during submission and cancels once the ID arrives`, async () => {
    const submitted = deferred<JobSnapshot>(); const terminal = deferred<JobSnapshot>();
    const cancelJob = vi.fn(async () => ({ id: 'late', status: 'running' } as JobSnapshot));
    const client = supervised({ submitJob: () => submitted.promise, cancelJob, getJob: () => terminal.promise });
    const run = new GraphExecution(); run.connect(client);
    const pending = run.start(request()); await flush();
    if (edit === 'graph') run.editGraph(); else run.editInputs();
    expect(run.state.cancellationRequested).toBe(true); expect(run.state.result).toBeNull();
    submitted.resolve({ id: 'late', status: 'queued' }); await flush();
    expect(cancelJob).toHaveBeenCalledExactlyOnceWith('late');
    expect(run.state.status).not.toBe('cancelled');
    terminal.resolve({ id: 'late', status: 'cancelled' }); await pending;
    expect(run.state.status).toBe('discarded');
  });
}
it('edits before validation completes prevent job submission', async () => {
  const validation = deferred<void>(); const client = supervised({ validateGraph: () => validation.promise });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush();
  run.editGraph(); validation.resolve(); await pending;
  expect(client.submitJob).not.toHaveBeenCalled(); expect(run.state.status).toBe('discarded');
});
it('edits before capabilities arrive prevent validation and submission', async () => {
  const capabilities = deferred<Awaited<ReturnType<RxchefClient['capabilities']>>>();
  const client = supervised({ capabilities: () => capabilities.promise, validateGraph: vi.fn(async () => {}) });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); run.editInputs();
  capabilities.resolve({ ...fixture.capabilities, jobs: true }); await pending;
  expect(client.validateGraph).not.toHaveBeenCalled(); expect(client.submitJob).not.toHaveBeenCalled();
});
it('cancels immediately after dispatch, deduplicates repeated edits and waits for worker termination', async () => {
  const terminal = deferred<JobSnapshot>(); const client = supervised({ getJob: () => terminal.promise });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush();
  run.editGraph(); run.editInputs(); await run.cancel();
  expect(client.cancelJob).toHaveBeenCalledTimes(1); expect(run.state.status).not.toBe('cancelled');
  terminal.resolve({ id: 'job', status: 'cancelled' }); await pending;
  expect(run.state.status).toBe('discarded');
});
it('failed remote cancellation keeps observing the worker and discards its late success', async () => {
  const submitted = deferred<JobSnapshot>(); const terminal = deferred<JobSnapshot>();
  const client = supervised({ submitJob: () => submitted.promise, getJob: () => terminal.promise,
    cancelJob: vi.fn(async () => { throw new Error('Remote cancellation unavailable'); }) });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush();
  run.editGraph(); submitted.resolve({ id: 'job', status: 'queued' }); await flush();
  expect(run.state.error).toContain('Remote cancellation unavailable');
  expect(run.state.canCancel).toBe(true); expect(run.state.status).not.toBe('cancelled');
  terminal.resolve({ id: 'job', status: 'succeeded', result: { kind: 'graph', value: response() } }); await pending;
  expect(run.state.status).toBe('discarded'); expect(run.state.result).toBeNull();
});
it('explicit cancellation only reports cancelled after a terminal snapshot', async () => {
  const terminal = deferred<JobSnapshot>(); const client = supervised({ getJob: () => terminal.promise });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush();
  await run.cancel(); expect(run.state.cancellationRequested).toBe(true);
  expect(run.state.status).not.toBe('cancelled');
  terminal.resolve({ id: 'job', status: 'cancelled' }); await pending; expect(run.state.status).toBe('cancelled');
});
it('connection replacement during submit cancels the old client without replacing the new result', async () => {
  const submitted = deferred<JobSnapshot>(); const client = supervised({ submitJob: () => submitted.promise });
  const run = new GraphExecution(); run.connect(client); const pending = run.start(request()); await flush();
  run.connect(local(async () => response())); await run.start(request());
  submitted.resolve({ id: 'job', status: 'queued' }); await pending;
  expect(client.cancelJob).toHaveBeenCalledExactlyOnceWith('job'); expect(run.state.status).toBe('succeeded');
});
