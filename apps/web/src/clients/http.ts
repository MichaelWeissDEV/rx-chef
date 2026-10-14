import {
  Capabilities,
  EngineTask,
  ClientError,
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
import { parseContract } from '../lib/contracts';

export class HttpRxchefClient implements RxchefClient {
  private baseUrl: string;
  private token: string | null;

  constructor(baseUrl: string = '', token: string | null = null) {
    this.baseUrl = baseUrl.replace(/\/+$/, '');
    this.token = token;
  }

  setToken(token: string | null) {
    this.token = token;
  }

  private async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const url = `${this.baseUrl}/api/v1/${path.replace(/^\/+/, '')}`;
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Accept: 'application/json',
      ...(options.headers as Record<string, string>),
    };

    if (this.token) {
      headers['Authorization'] = `Bearer ${this.token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
      redirect: 'error',
      cache: 'no-store',
    });

    if (!response.ok) {
      let errorData: ClientError;
      try {
        errorData = parseContract('error', await response.json());
      } catch {
        errorData = {
          kind: response.status === 401 ? 'unauthorized' : 'transport',
          message: `HTTP ${response.status} ${response.statusText}`,
        };
      }
      throw errorData;
    }

    if (response.status === 204) {
      return null as unknown as T;
    }

    return response.json();
  }

  async validateGraph(request: import('../types').GraphRequest): Promise<void> {
    await this.request('graphs/validate', { method: 'POST', body: JSON.stringify(request) });
  }
  async executeGraph(request: import('../types').GraphRequest) {
    return parseContract('graph_response', await this.request('graphs/execute', { method: 'POST', body: JSON.stringify(request) }));
  }
  async capabilities(): Promise<Capabilities> {
    return parseContract('capabilities', await this.request('capabilities'));
  }

  async operations(): Promise<OperationDescriptor[]> {
    return parseContract('operations', await this.request('operations'));
  }

  async describe(id: string): Promise<OperationDescriptor> {
    return parseContract('operations', [await this.request(`operations/${encodeURIComponent(id)}`)])[0];
  }

  async validate(request: ExecuteRequest): Promise<void> {
    await this.request<void>('validate', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async execute(request: ExecuteRequest): Promise<ExecuteResponse> {
    return parseContract('response', await this.request('execute', {
      method: 'POST',
      body: JSON.stringify(request),
    }));
  }

  async magic(request: MagicRequest): Promise<MagicCandidate[]> {
    return parseContract('magic', await this.request('magic', {
      method: 'POST',
      body: JSON.stringify(request),
    }));
  }

  async scan(request: ScanRequest): Promise<ScanFinding[]> {
    return parseContract('scan', await this.request('scan', {
      method: 'POST',
      body: JSON.stringify(request),
    }));
  }

  async submitJob(task: EngineTask): Promise<JobSnapshot> {
    return parseContract('job', await this.request('jobs', {
      method: 'POST',
      body: JSON.stringify(task),
    }));
  }

  async getJob(id: string): Promise<JobSnapshot> {
    return parseContract('job', await this.request(`jobs/${encodeURIComponent(id)}`));
  }

  async cancelJob(id: string): Promise<JobSnapshot> {
    return parseContract('job', await this.request(`jobs/${encodeURIComponent(id)}/cancel`, {
      method: 'POST',
    }));
  }
}
