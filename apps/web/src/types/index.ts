// Wire models are generated from the authoritative Rust serializers.
export * from '../generated/contracts';
export type { ArgumentDescriptor as ArgSchema, Finding as ScanFinding } from '../generated/contracts';
import type { Capabilities, ExecuteRequest, ExecuteResponse, JobSnapshot, MagicCandidate, MagicRequest,
  OperationDescriptor, Finding as ScanFinding, ScanRequest, RecipeStep as WireStep } from '../generated/contracts';
/** UI-only disable property is never sent to the engine. */
export type RecipeStep = Omit<WireStep, 'args'> & { args: string[]; disabled?: boolean };
export interface RxchefClient {
  capabilities(): Promise<Capabilities>;
  operations(): Promise<OperationDescriptor[]>;
  describe(id: string): Promise<OperationDescriptor>;
  validate(request: ExecuteRequest): Promise<void>;
  execute(request: ExecuteRequest): Promise<ExecuteResponse>;
  magic(request: MagicRequest): Promise<MagicCandidate[]>;
  scan(request: ScanRequest): Promise<ScanFinding[]>;
  submitJob(task: { method: string; params: unknown }): Promise<JobSnapshot>;
  getJob(id: string): Promise<JobSnapshot>;
  cancelJob(id: string): Promise<JobSnapshot>;
}
