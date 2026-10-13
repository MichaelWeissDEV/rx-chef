import {
  Capabilities,
  ExecuteRequest,
  ExecuteResponse,
  JobSnapshot,
  MagicCandidate,
  MagicRequest,
  OperationDescriptor,
  RxchefClient,
  ScanFinding,
  ScanRequest,
} from '../types';

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
    __TAURI__?: {
      core: {
        invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
      };
    };
  }
}

export class TauriRxchefClient implements RxchefClient {
  private invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    if (window.__TAURI__?.core?.invoke) {
      return window.__TAURI__.core.invoke<T>(cmd, args);
    }
    return Promise.reject(new Error('Tauri IPC environment is not available'));
  }

  async capabilities(): Promise<Capabilities> {
    return this.invoke<Capabilities>('plugin:rxchef|capabilities');
  }

  async operations(): Promise<OperationDescriptor[]> {
    return this.invoke<OperationDescriptor[]>('plugin:rxchef|operations');
  }

  async describe(id: string): Promise<OperationDescriptor> {
    return this.invoke<OperationDescriptor>('plugin:rxchef|describe', { id });
  }

  async validate(request: ExecuteRequest): Promise<void> {
    return this.invoke<void>('plugin:rxchef|validate', { request });
  }

  async execute(request: ExecuteRequest): Promise<ExecuteResponse> {
    return this.invoke<ExecuteResponse>('plugin:rxchef|execute', { request });
  }

  async magic(request: MagicRequest): Promise<MagicCandidate[]> {
    return this.invoke<MagicCandidate[]>('plugin:rxchef|magic', { request });
  }

  async scan(request: ScanRequest): Promise<ScanFinding[]> {
    return this.invoke<ScanFinding[]>('plugin:rxchef|scan', { request });
  }

  async submitJob(task: { method: string; params: unknown }): Promise<JobSnapshot> {
    return this.invoke<JobSnapshot>('plugin:rxchef|submit_job', { task });
  }

  async getJob(id: string): Promise<JobSnapshot> {
    return this.invoke<JobSnapshot>('plugin:rxchef|get_job', { id });
  }

  async cancelJob(id: string): Promise<JobSnapshot> {
    return this.invoke<JobSnapshot>('plugin:rxchef|cancel_job', { id });
  }
}
