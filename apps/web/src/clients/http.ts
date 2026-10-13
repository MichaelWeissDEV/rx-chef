import {
  Capabilities,
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
    });

    if (!response.ok) {
      let errorData: ClientError;
      try {
        errorData = await response.json();
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

  async capabilities(): Promise<Capabilities> {
    return this.request<Capabilities>('capabilities');
  }

  async operations(): Promise<OperationDescriptor[]> {
    return this.request<OperationDescriptor[]>('operations');
  }

  async describe(id: string): Promise<OperationDescriptor> {
    return this.request<OperationDescriptor>(`operations/${encodeURIComponent(id)}`);
  }

  async validate(request: ExecuteRequest): Promise<void> {
    await this.request<void>('validate', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async execute(request: ExecuteRequest): Promise<ExecuteResponse> {
    return this.request<ExecuteResponse>('execute', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async magic(request: MagicRequest): Promise<MagicCandidate[]> {
    return this.request<MagicCandidate[]>('magic', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async scan(request: ScanRequest): Promise<ScanFinding[]> {
    return this.request<ScanFinding[]>('scan', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async submitJob(task: { method: string; params: any }): Promise<JobSnapshot> {
    return this.request<JobSnapshot>('jobs', {
      method: 'POST',
      body: JSON.stringify(task),
    });
  }

  async getJob(id: string): Promise<JobSnapshot> {
    return this.request<JobSnapshot>(`jobs/${encodeURIComponent(id)}`);
  }

  async cancelJob(id: string): Promise<JobSnapshot> {
    return this.request<JobSnapshot>(`jobs/${encodeURIComponent(id)}/cancel`, {
      method: 'POST',
    });
  }
}
