/* Generated from Rust serialization. Run npm run contracts. Do not edit. */

/**
 * Whether an operation consumes input bytes.
 */
export type InputRequirement = "required" | "optional" | "ignored";
/**
 * Verification state of an operation implementation.
 */
export type ImplementationStatus = "complete" | "partial" | "unsupported" | "experimental";
/**
 * Whether an implementation can run in the current build and platform.
 */
export type Availability = "available" | "feature_disabled" | "platform_unavailable";
/**
 * Externally observable behavior an operation may perform.
 */
export type SideEffect =
  "network" | "filesystem-read" | "filesystem-write" | "random" | "time" | "external-process" | "native-library";
/**
 * Compatibility level with upstream CyberChef behavior.
 */
export type ParityStatus = "exact" | "compatible" | "intentional_difference" | "unknown" | "not_applicable";
/**
 * Machine-readable argument value category.
 */
export type ArgKind =
  | "string"
  | "integer"
  | "unsigned_integer"
  | "float"
  | "boolean"
  | "bytes"
  | "hex_bytes"
  | "base64_bytes"
  | "enum"
  | "regex"
  | "path"
  | "url";
export type ErrorKind =
  | "invalid_input"
  | "invalid_recipe"
  | "unknown_operation"
  | "unavailable"
  | "invalid_argument"
  | "operation"
  | "limit_exceeded"
  | "policy_denied"
  | "transport"
  | "protocol"
  | "busy"
  | "not_found"
  | "cancelled"
  | "timeout"
  | "unauthorized";
export type JobStatus = "queued" | "running" | "succeeded" | "failed" | "cancelled" | "timed_out";
export type TaskResult =
  | {
      kind: "execution";
      value: ExecuteResponse;
    }
  | {
      kind: "validated";
    }
  | {
      kind: "magic";
      value: MagicCandidate[];
    }
  | {
      kind: "scan";
      value: Finding[];
    };
/**
 * Shared job payload, also used for supervised local worker execution.
 */
export type EngineTask =
  | {
      method: "execute";
      params: ExecuteRequest;
    }
  | {
      method: "validate";
      params: ExecuteRequest;
    }
  | {
      method: "magic";
      params: MagicRequest;
    }
  | {
      method: "scan";
      params: ScanRequest;
    };

export interface Contracts {
  operations: OperationDescriptor[];
  argument: ArgumentDescriptor;
  result: ExecutionResult;
  request: ExecuteRequest;
  response: ExecuteResponse;
  capabilities: Capabilities;
  magic_request: MagicRequest;
  magic: MagicCandidate[];
  scan_request: ScanRequest;
  scan: Finding[];
  error: ClientError;
  job: JobSnapshot;
  task: EngineTask;
}
/**
 * Serializable operation metadata used by CLI and editor integrations.
 */
export interface OperationDescriptor {
  name: string;
  id: string;
  aliases: string[];
  module: string;
  description: string;
  input_type: string;
  output_type: string;
  broken: boolean;
  input_requirement: InputRequirement;
  implementation_status: ImplementationStatus;
  availability: Availability;
  feature_requirements: string[];
  platform_requirements: string[];
  side_effects: SideEffect[];
  deterministic: boolean;
  parity: ParityStatus;
  known_limitations: string[];
  documentation_slug: string;
  args: ArgumentDescriptor[];
}
/**
 * One operation argument in a machine-readable descriptor.
 */
export interface ArgumentDescriptor {
  name: string;
  description: string;
  default: string;
  kind: ArgKind;
  required: boolean;
  choices: string[];
  minimum?: string | null;
  maximum?: string | null;
  sensitive: boolean;
}
/**
 * Binary-safe result envelope.
 */
export interface ExecutionResult {
  /**
   * UTF-8-lossy convenience representation for editor UIs.
   */
  output: string;
  /**
   * Exact output bytes encoded as standard padded Base64.
   */
  output_base64: string;
  output_len: number;
  /**
   * Whether `output` is an exact UTF-8 representation (`false` means it is
   * only a lossy UI convenience and `output_base64` is authoritative).
   */
  output_is_utf8: boolean;
}
export interface ExecuteRequest {
  /**
   * None means no input source; Some("") means explicitly empty input.
   */
  input_base64?: string | null;
  recipe: RecipeStep[];
  variables?: {
    [k: string]: string;
  };
  /**
   * Opt-in: trace payloads can contain sensitive data.
   */
  trace?: TraceLimits | null;
}
/**
 * One operation and its ordered argument values in a recipe.
 */
export interface RecipeStep {
  /**
   * Operation name or normalized identifier.
   */
  op: string;
  /**
   * Ordered raw arguments. Parsing is delegated to the shared registry
   * runtime after variable and register expansion.
   */
  args?: string[];
}
export interface TraceLimits {
  max_entries: number;
  max_bytes_per_entry: number;
  max_total_bytes: number;
}
export interface ExecuteResponse {
  result: ExecutionResult;
  trace: TraceArtifact[];
  trace_truncated: boolean;
}
export interface TraceArtifact {
  /**
   * Zero-based index into the original recipe, including repeated steps.
   */
  step_index: number;
  operation: string;
  input_bytes: number;
  output_bytes: number;
  elapsed_ms: number;
  control_flow: boolean;
  /**
   * Exact prefix, never a reconstruction from a lossy text preview.
   */
  output_base64: string;
  truncated: boolean;
}
export interface Capabilities {
  version: string;
  protocol_version: number;
  binary_transport: string;
  trace: boolean;
  /**
   * True when the endpoint offers process-supervised, cancellable jobs.
   */
  jobs: boolean;
  max_input_bytes: number;
  max_output_bytes: number;
  max_steps: number;
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
/**
 * A single hit produced by the scanner.
 */
export interface Finding {
  /**
   * Byte offset of the token in the original stream.
   */
  offset: number;
  /**
   * Length of the matched token in bytes.
   */
  len: number;
  /**
   * Encodings that fired on the token (e.g. `["From Base64"]`).
   */
  kinds: string[];
  /**
   * Shannon entropy of the token bytes.
   */
  entropy: number;
  /**
   * The matched token text (length-capped preview).
   */
  token: string;
  /**
   * Best decoded preview, when `decode` was requested and a decode succeeded.
   */
  decoded?: string | null;
  /**
   * Recipe that produced `decoded`.
   */
  recipe?: RecipeStep2[] | null;
}
/**
 * One decode step in a recovered recipe.
 */
export interface RecipeStep2 {
  op: string;
  args: string[];
}
/**
 * Stable classification independent of human-readable messages.
 */
export interface ClientError {
  kind: ErrorKind;
  message: string;
  step?: number | null;
}
export interface JobSnapshot {
  id: string;
  status: JobStatus;
  /**
   * No fabricated percentage: None when operation progress is unknown.
   */
  progress?: number | null;
  result?: TaskResult | null;
  error?: ClientError | null;
}
