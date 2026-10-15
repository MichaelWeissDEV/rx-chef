import type { Capabilities, GraphRequest, GraphResult, RxchefClient, GraphNodeProgress } from '../../types';
import { executeTask } from '../../lib/jobs';
import { errorMessage, parseGraphResponse } from '../../lib/contracts';
export type RunStatus = 'idle' | 'starting' | 'queued' | 'running' | 'succeeded' | 'failed' | 'cancelled' | 'timed_out' | 'discarded';
export interface RunSnapshot {
  readonly run_id: string;
  readonly graph_revision: number;
  readonly input_revision: number;
  readonly connection_revision: number;
  readonly request: GraphRequest;
  readonly execution_policy: 'engine-enforced';
  readonly started_at: string;
}
export interface GraphRunState {
  status: RunStatus; snapshot?: RunSnapshot; capabilities?: Capabilities;
  result: GraphResult | null; error?: string; notice?: string; canCancel: boolean;
  nodes?: GraphNodeProgress[];
  cancellationRequested?: boolean;
}
interface Run { snapshot: RunSnapshot; client: RxchefClient; job?: string; cancelled: boolean; cancellation?: Promise<void> }
function freeze<T>(value: T): T {
  if (value && typeof value === 'object') { Object.values(value).forEach(freeze); Object.freeze(value); }
  return value;
}
/** Coordinates identity and transport lifecycle only. Execution stays in Rust. */
export class GraphExecution {
  private graphRevision = 0;
  private inputRevision = 0;
  private connectionRevision = 0;
  private sequence = 0;
  private client?: RxchefClient;
  private active?: Run;
  private listener: (state: GraphRunState) => void = () => {};
  state: GraphRunState = { status: 'idle', result: null, canCancel: false };
  subscribe(listener: (state: GraphRunState) => void) { this.listener = listener; listener(this.state); return () => { this.listener = () => {}; }; }
  private emit(state: GraphRunState) { this.state = state; this.listener(state); }
  connect(client: RxchefClient) {
    if (this.client === client) return;
    this.close(); this.client = client; this.connectionRevision++;
    this.emit({ status: 'idle', result: null, canCancel: false });
  }
  close() {
    const old = this.active; this.active = undefined;
    if (old) void this.cancelRun(old).catch(() => {});
    this.client = undefined;
  }
  editGraph() { this.graphRevision++; this.invalidate(); }
  editInputs() { this.inputRevision++; this.invalidate(); }
  private invalidate() {
    const run = this.active;
    this.emit({ ...this.state, result: null, nodes:[], cancellationRequested: !!run,
      notice: run ? 'Graph or inputs changed; cancellation requested for the earlier revision.' : undefined });
    if (run) void this.cancelRun(run).catch(error => {
      if (this.current(run)) this.emit({ ...this.state, error: errorMessage(error) });
    });
  }
  private current(run: Run) { return this.active === run && this.client === run.client; }
  private matches(run: Run) {
    return this.current(run) && run.snapshot.graph_revision === this.graphRevision && run.snapshot.input_revision === this.inputRevision && run.snapshot.connection_revision === this.connectionRevision;
  }
  private async cancelRun(run: Run) {
    run.cancelled = true;
    if (run.job && !run.cancellation) {
      run.cancellation = run.client.cancelJob(run.job).then(() => {}).catch(error => {
        run.cancellation = undefined;
        throw error;
      });
    }
    await run.cancellation;
  }
  async cancel() {
    const run = this.active; if (!run) return;
    this.emit({ ...this.state, cancellationRequested: true, notice: 'Cancellation requested; waiting for execution to end.' });
    try { await this.cancelRun(run); }
    catch (error) { if (this.current(run)) this.emit({ ...this.state, error: errorMessage(error) }); }
  }
  async start(request: GraphRequest) {
    const client = this.client; if (!client) return;
    if (this.active) void this.cancelRun(this.active).catch(() => {});
    const snapshot = freeze(structuredClone({ run_id: `graph-run-${++this.sequence}`, graph_revision: this.graphRevision,
      input_revision: this.inputRevision, connection_revision: this.connectionRevision,
      request, execution_policy: 'engine-enforced' as const, started_at: new Date().toISOString() }));
    const run: Run = { snapshot, client, cancelled: false }; this.active = run;
    this.emit({ status: 'starting', snapshot, result: null, canCancel: true });
    try {
      const capabilities = await client.capabilities();
      if (!this.current(run)) return;
      if (!capabilities.graphs) throw new Error('Engine does not support graphs.');
      if (run.cancelled) { this.emit({ status: this.matches(run) ? 'cancelled' : 'discarded', snapshot, result: null, canCancel: false }); return; }
      this.emit({ status: 'queued', snapshot, capabilities, result: null, canCancel: capabilities.jobs });
      await client.validateGraph(snapshot.request);
      if (!this.current(run)) return;
      if (run.cancelled) { this.emit({ status: this.matches(run) ? 'cancelled' : 'discarded', snapshot, result: null, canCancel: false }); return; }
      const outcome = await executeTask(client, { method: 'graph_execute', params: snapshot.request }, {
        current: () => this.current(run), cancelled: () => run.cancelled,
        requestCancel: () => this.cancelRun(run),
        cancellationError: error => { if (this.current(run)) this.emit({ ...this.state, error: errorMessage(error) }); },
        job: id => { run.job = id; }, status: status => { if (this.current(run)) this.emit({ ...this.state, status }); },
        snapshot: snapshot => {if(this.matches(run))this.emit({...this.state,nodes:snapshot.graph_nodes??[]});},
      }, capabilities);
      if (outcome.kind !== 'graph') throw new Error('Job returned no graph result.');
      const result = outcome.value;
      if (!this.current(run)) return;
      if (!this.matches(run) || run.cancelled) {
        this.emit({ status: 'discarded', snapshot, result: null, canCancel: false, notice: 'Result discarded: graph, inputs, or cancellation changed during execution.' }); return;
      }
      this.emit({ status: 'succeeded', snapshot, capabilities, result: parseGraphResponse(result), canCancel: false });
    } catch (error) {
      if (this.current(run)) { const kind = error && typeof error === 'object' && 'kind' in error ? error.kind : ''; this.emit({ status: !this.matches(run) ? 'discarded' : kind === 'cancelled' ? 'cancelled' : kind === 'timeout' ? 'timed_out' : 'failed', snapshot, result: null, error: errorMessage(error), canCancel: false }); }
    } finally { if (this.active === run) this.active = undefined; }
  }
}
