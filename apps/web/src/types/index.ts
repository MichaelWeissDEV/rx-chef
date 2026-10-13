// Shared TypeScript contracts for rx-chef matching Rust crates/client and crates/server

export type ErrorKind =
  | 'invalid_input'
  | 'invalid_recipe'
  | 'unknown_operation'
  | 'unavailable'
  | 'invalid_argument'
  | 'operation'
  | 'limit_exceeded'
  | 'policy_denied'
  | 'transport'
  | 'protocol'
  | 'busy'
  | 'not_found'
  | 'cancelled'
  | 'timeout'
  | 'unauthorized';

export interface ClientError {
  kind: ErrorKind;
  message: string;
  step?: number | null;
}

export interface Capabilities {
  version: string;
  protocol_version: number;
  binary_transport: string;
  trace: boolean;
  jobs: boolean;
  max_input_bytes: number;
  max_output_bytes: number;
  max_steps: number;
}

export interface ArgChoice {
  value: string;
  label: string;
}

export interface ArgSchema {
  name: string;
  description: string;
  default_value: string;
  kind: 'string' | 'bytes' | 'integer' | 'unsigned_integer' | 'boolean' | 'choice' | 'text';
  required: boolean;
  choices?: string[];
  sensitive?: boolean;
}

export interface OperationDescriptor {
  id: string;
  name: string;
  description: string;
  category: string;
  args: ArgSchema[];
  input_type?: string;
  output_type?: string;
  side_effects: string[];
  authoritative: boolean;
}

export interface RecipeStep {
  op: string;
  args: string[];
  disabled?: boolean;
}

export interface TraceLimits {
  max_entries: number;
  max_bytes_per_entry: number;
  max_total_bytes: number;
}

export interface ExecuteRequest {
  input_base64?: string | null;
  recipe: RecipeStep[];
  variables?: Record<string, string>;
  trace?: TraceLimits | null;
}

export interface TraceArtifact {
  step_index: number;
  operation: string;
  input_bytes: number;
  output_bytes: number;
  elapsed_ms: number;
  control_flow: boolean;
  output_base64: string;
  truncated: boolean;
}

export interface ExecutionResult {
  output_base64: string;
  output_len: number;
  preview: string;
  is_text: boolean;
}

export interface ExecuteResponse {
  result: ExecutionResult;
  trace: TraceArtifact[];
  trace_truncated: boolean;
}

export interface MagicRequest {
  input_base64: string;
  depth?: number;
  intensive?: boolean;
  max_results?: number;
  crib?: string | null;
}

export interface MagicCandidate {
  recipe: RecipeStep[];
  result: ExecutionResult;
  score: number;
  entropy: number;
  printable_ratio: number;
  matched_crib: boolean;
}

export interface ScanRequest {
  input_base64: string;
  min_len?: number;
  max_findings?: number;
  decode?: boolean;
  depth?: number;
  crib?: string | null;
}

export interface ScanFinding {
  offset: number;
  length: number;
  protocol: string;
  value: string;
  decoded?: string | null;
}

export type JobStatus = 'queued' | 'running' | 'succeeded' | 'failed' | 'cancelled' | 'timed_out';

export interface JobSnapshot {
  id: string;
  status: JobStatus;
  progress?: number | null;
  result?: {
    kind: string;
    value: any;
  } | null;
  error?: ClientError | null;
}

export interface RxchefClient {
  capabilities(): Promise<Capabilities>;
  operations(): Promise<OperationDescriptor[]>;
  describe(id: string): Promise<OperationDescriptor>;
  validate(request: ExecuteRequest): Promise<void>;
  execute(request: ExecuteRequest): Promise<ExecuteResponse>;
  magic(request: MagicRequest): Promise<MagicCandidate[]>;
  scan(request: ScanRequest): Promise<ScanFinding[]>;
  submitJob(task: { method: string; params: any }): Promise<JobSnapshot>;
  getJob(id: string): Promise<JobSnapshot>;
  cancelJob(id: string): Promise<JobSnapshot>;
}
