// Wire models are generated from the authoritative Rust serializers.
export * from '../generated/contracts';
export type { ArgumentDescriptor as ArgSchema, Finding as ScanFinding } from '../generated/contracts';
import type { Capabilities, ExecuteRequest, ExecuteResponse, JobSnapshot, MagicCandidate, MagicRequest,
  EngineTask, OperationDescriptor, Finding as ScanFinding, ScanRequest, RecipeStep as WireStep } from '../generated/contracts';
/** UI-only disable property is never sent to the engine. */
export type RecipeStep = Omit<WireStep, 'args'> & { args: string[]; disabled?: boolean; breakpoint?: boolean };
export interface RxchefClient {
  validateProject(project: import('../generated/contracts').WorkbenchProject): Promise<import('../generated/contracts').WorkbenchProject>;
  validateGraph(request: import('../generated/contracts').GraphRequest): Promise<void>;
  executeGraph(request: import('../generated/contracts').GraphRequest): Promise<import('../generated/contracts').GraphResult>;
  capabilities(): Promise<Capabilities>;
  operations(): Promise<OperationDescriptor[]>;
  describe(id: string): Promise<OperationDescriptor>;
  validate(request: ExecuteRequest): Promise<void>;
  execute(request: ExecuteRequest): Promise<ExecuteResponse>;
  magic(request: MagicRequest): Promise<MagicCandidate[]>;
  scan(request: ScanRequest): Promise<ScanFinding[]>;
  submitJob(task: EngineTask): Promise<JobSnapshot>;
  getJob(id: string): Promise<JobSnapshot>;
  cancelJob(id: string): Promise<JobSnapshot>;
}
