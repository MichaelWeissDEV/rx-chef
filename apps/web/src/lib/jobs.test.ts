import { afterEach, describe, expect, it, vi } from 'vitest';
import { executeTask, type TaskControl } from './jobs';
import { parseContract } from './contracts';
import type { Capabilities, EngineTask, JobSnapshot, RxchefClient } from '../types';
import fixture from '../generated/contracts.fixture.json';

const capabilities = (jobs = true): Capabilities => ({ ...fixture.capabilities, jobs });
const task = (): EngineTask => ({ method: 'graph_execute', params: parseContract('graph_request', structuredClone(fixture.graph_request)) });
const result = () => ({ kind: 'graph' as const, value: parseContract('graph_response', structuredClone(fixture.graph_response)) });
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(done => { resolve = done; });
  return { promise, resolve };
}
function control(overrides: Partial<TaskControl> = {}): TaskControl {
  return { current: () => true, cancelled: () => false, job: vi.fn(), status: vi.fn(), snapshot: vi.fn(), ...overrides };
}
function client(overrides: Partial<RxchefClient> = {}): RxchefClient {
  return {
    submitJob: vi.fn(async () => ({ id: 'submitted', status: 'queued' })),
    getJob: vi.fn(async () => ({ id: 'submitted', status: 'succeeded', result: result() })),
    cancelJob: vi.fn(async () => ({ id: 'submitted', status: 'running' })),
    execute: vi.fn(async () => structuredClone(fixture.response)),
    executeGraph: vi.fn(async () => structuredClone(fixture.graph_response)),
    ...overrides,
  } as unknown as RxchefClient;
}
const flush = async () => { for (let i = 0; i < 8; i++) await Promise.resolve(); };
afterEach(() => vi.useRealTimers());

describe('dispatch boundaries', () => {
  it.each(['superseded', 'cancelled'])('does not dispatch a %s task', async reason => {
    const transport = client(); const owner = control({ current: () => reason !== 'superseded', cancelled: () => reason === 'cancelled' });
    await expect(executeTask(transport, task(), owner, capabilities())).rejects.toMatchObject({ kind: 'cancelled' });
    expect(transport.submitJob).not.toHaveBeenCalled(); expect(transport.executeGraph).not.toHaveBeenCalled();
    expect(owner.job).not.toHaveBeenCalled();
  });
  it('routes an unsupervised graph through the existing engine client', async () => {
    const transport = client(); const owner = control(); const request = task();
    await expect(executeTask(transport, request, owner, capabilities(false))).resolves.toEqual(result());
    expect(transport.executeGraph).toHaveBeenCalledWith(request.params);
    expect(transport.submitJob).not.toHaveBeenCalled(); expect(owner.status).toHaveBeenCalledExactlyOnceWith('running');
  });
  it('routes an unsupervised recipe without accidentally dispatching a graph', async () => {
    const transport = client(); const request: EngineTask = { method: 'execute', params: structuredClone(fixture.request) };
    await expect(executeTask(transport, request, control(), capabilities(false))).resolves.toEqual({ kind: 'execution', value: fixture.response });
    expect(transport.execute).toHaveBeenCalledWith(request.params); expect(transport.executeGraph).not.toHaveBeenCalled();
  });
  it('does not emulate unsupported tasks when job capability is missing', async () => {
    const transport = client(); const scan: EngineTask = { method: 'scan', params: structuredClone(fixture.scan_request) };
    await expect(executeTask(transport, scan, control(), capabilities(false))).rejects.toThrow('requires supervised jobs');
    expect(transport.submitJob).not.toHaveBeenCalled(); expect(transport.execute).not.toHaveBeenCalled();
  });
  it('propagates submit failure without inventing a job identity or polling', async () => {
    const error = { kind: 'busy', message: 'Admission limit reached' }; const owner = control();
    const transport = client({ submitJob: vi.fn(async () => { throw error; }) });
    await expect(executeTask(transport, task(), owner, capabilities())).rejects.toBe(error);
    expect(owner.job).not.toHaveBeenCalled(); expect(transport.getJob).not.toHaveBeenCalled();
  });
});

describe('supervised status observation', () => {
  it('publishes queued/running snapshots in order and returns the authoritative result', async () => {
    vi.useFakeTimers(); const owner = control();
    const queued: JobSnapshot = { id: 'submitted', status: 'queued' };
    const running: JobSnapshot = { id: 'submitted', status: 'running' };
    const succeeded: JobSnapshot = { id: 'submitted', status: 'succeeded', result: result() };
    const transport = client({ getJob: vi.fn().mockResolvedValueOnce(queued).mockResolvedValueOnce(running).mockResolvedValueOnce(succeeded) });
    const pending = executeTask(transport, task(), owner, capabilities());
    await vi.advanceTimersByTimeAsync(200); await expect(pending).resolves.toEqual(result());
    expect(owner.job).toHaveBeenCalledExactlyOnceWith('submitted');
    expect(vi.mocked(owner.status).mock.calls.map(call => call[0])).toEqual(['queued', 'running']);
    expect(vi.mocked(owner.snapshot!).mock.calls.map(call => call[0])).toEqual([queued, running, succeeded]);
    expect(transport.cancelJob).not.toHaveBeenCalled();
  });
  it.each([['failed', 'failed'], ['cancelled', 'cancelled'], ['timed_out', 'timeout']] as const)(
    'maps terminal %s to a typed error without polling again', async (status, kind) => {
      const transport = client({ getJob: vi.fn(async () => ({ id: 'submitted', status })) });
      await expect(executeTask(transport, task(), control(), capabilities())).rejects.toMatchObject({ kind });
      expect(transport.getJob).toHaveBeenCalledTimes(1);
    });
  it('preserves the engine error rather than replacing its step and message', async () => {
    const error = { kind: 'operation' as const, message: 'Execution failed', step: 3 };
    const transport = client({ getJob: vi.fn(async (): Promise<JobSnapshot> => ({ id: 'submitted', status: 'failed', error })) });
    await expect(executeTask(transport, task(), control(), capabilities())).rejects.toBe(error);
  });
  it('rejects a successful terminal snapshot without a result', async () => {
    const transport = client({ getJob: vi.fn(async (): Promise<JobSnapshot> => ({ id: 'submitted', status: 'succeeded' })) });
    await expect(executeTask(transport, task(), control(), capabilities())).rejects.toThrow('no result');
  });
  it('propagates a polling transport failure without reporting terminal success', async () => {
    const error = { kind: 'transport', message: 'Connection closed' }; const owner = control();
    const transport = client({ getJob: vi.fn(async () => { throw error; }) });
    await expect(executeTask(transport, task(), owner, capabilities())).rejects.toBe(error);
    expect(owner.snapshot).not.toHaveBeenCalled();
  });
});

describe('cancellation and ownership races', () => {
  it('cancels a late submission after ownership ends and never observes its result', async () => {
    const submitted = deferred<JobSnapshot>(); let current = true; const owner = control({ current: () => current });
    const transport = client({ submitJob: () => submitted.promise });
    const pending = executeTask(transport, task(), owner, capabilities()).catch(error => error);
    await flush(); current = false; submitted.resolve({ id: 'late', status: 'queued' });
    expect(await pending).toMatchObject({ kind: 'cancelled' });
    expect(transport.cancelJob).toHaveBeenCalledExactlyOnceWith('late');
    expect(transport.getJob).not.toHaveBeenCalled(); expect(owner.snapshot).not.toHaveBeenCalled();
  });
  it('keeps waiting when cancellation acknowledges a still-running worker', async () => {
    const submitted = deferred<JobSnapshot>(); const terminal = deferred<JobSnapshot>(); let cancelled = false;
    const owner = control({ cancelled: () => cancelled });
    const transport = client({ submitJob: () => submitted.promise, getJob: () => terminal.promise });
    let settled = false;
    const pending = executeTask(transport, task(), owner, capabilities()).catch(error => { settled = true; return error; });
    await flush(); cancelled = true; submitted.resolve({ id: 'submitted', status: 'queued' }); await flush();
    expect(transport.cancelJob).toHaveBeenCalledTimes(1); expect(settled).toBe(false);
    terminal.resolve({ id: 'submitted', status: 'cancelled' });
    expect(await pending).toMatchObject({ kind: 'cancelled' });
  });
  it('uses the owner cancellation callback to deduplicate transport cancellation', async () => {
    const submitted = deferred<JobSnapshot>(); let cancelled = false; const requestCancel = vi.fn(async () => {});
    const transport = client({ submitJob: () => submitted.promise });
    const pending = executeTask(transport, task(), control({ cancelled: () => cancelled, requestCancel }), capabilities());
    await flush(); cancelled = true; submitted.resolve({ id: 'submitted', status: 'queued' }); await pending;
    expect(requestCancel).toHaveBeenCalledTimes(1); expect(transport.cancelJob).not.toHaveBeenCalled();
  });
  it('reports a failed cancellation request but keeps observing the live job', async () => {
    const submitted = deferred<JobSnapshot>(); let cancelled = false; const failure = new Error('Cancel transport failed');
    const cancellationError = vi.fn(); const transport = client({ submitJob: () => submitted.promise,
      cancelJob: vi.fn(async () => { throw failure; }) });
    const pending = executeTask(transport, task(), control({ cancelled: () => cancelled, cancellationError }), capabilities());
    await flush(); cancelled = true; submitted.resolve({ id: 'submitted', status: 'queued' });
    await expect(pending).resolves.toEqual(result());
    expect(cancellationError).toHaveBeenCalledExactlyOnceWith(failure);
    expect(transport.getJob).toHaveBeenCalledTimes(1);
  });
  it('suppresses a terminal snapshot if ownership changes while polling', async () => {
    const terminal = deferred<JobSnapshot>(); let current = true; const owner = control({ current: () => current });
    const transport = client({ getJob: () => terminal.promise });
    const pending = executeTask(transport, task(), owner, capabilities()).catch(error => error);
    await flush(); current = false; terminal.resolve({ id: 'submitted', status: 'succeeded', result: result() });
    expect(await pending).toMatchObject({ kind: 'cancelled' }); expect(owner.snapshot).not.toHaveBeenCalled();
  });
});
