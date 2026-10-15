import { invoke } from '@tauri-apps/api/core';
import type { ExecuteRequest, MagicRequest, ScanRequest, EngineTask, RxchefClient } from '../types';
import { parseContract, parseCapabilities } from '../lib/contracts';
declare global { interface Window { __TAURI_INTERNALS__?: unknown } }
export class TauriRxchefClient implements RxchefClient {
  async validateProject(project: import('../types').WorkbenchProject) { return parseContract('workbench_project', await this.call('project_validate', project)); }
  private call(method: string, params: unknown = null): Promise<unknown> { return invoke('engine_call', { method, params }); }
  async validateGraph(request: import('../types').GraphRequest): Promise<void> { await this.call('graph_validate', request); }
  async executeGraph(request: import('../types').GraphRequest) { return parseContract('graph_response', await this.call('graph_execute', request)); }
  async capabilities() { return parseCapabilities(await this.call('capabilities')); }
  async operations() { return parseContract('operations', await this.call('operations')); }
  async describe(id: string) { return parseContract('operations', [await this.call('describe', { id })])[0]; }
  async validate(request: ExecuteRequest): Promise<void> { await this.call('validate', request); }
  async execute(request: ExecuteRequest) { return parseContract('response', await this.call('execute', request)); }
  async magic(request: MagicRequest) { return parseContract('magic', await this.call('magic', request)); }
  async scan(request: ScanRequest) { return parseContract('scan', await this.call('scan', request)); }
  async submitJob(task: EngineTask) { return parseContract('job', await this.call('submit_job', task)); }
  async getJob(id: string) { return parseContract('job', await this.call('get_job', { id })); }
  async cancelJob(id: string) { return parseContract('job', await this.call('cancel_job', { id })); }
}
export const configureConnection = (url: string, token: string) => invoke('configure_connection', { url: url || null, token: token || null });
export const importNativeFile = (limit = 8 * 1024 * 1024) => invoke<string | null>('import_file', { limit });
export const exportNativeFile = (data: string, filename?: string) => invoke<boolean>('export_file', { data, filename: filename ?? null });
