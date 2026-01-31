/**
 * Pure TypeScript Memory Engine
 * Replaces Rust engine with native TypeScript implementation
 */

/**
 * Memory kinds - different types of memories with different scoring behaviors
 */
export enum MemoryKind {
  Semantic = "semantic", // facts / knowledge
  Episodic = "episodic", // events / experiences
  Working = "working", // short-term context
  Procedural = "procedural", // skills / behaviors
}

/**
 * Base memory fields shared across all memory types
 */
export interface BaseMemory {
  id: string;
  kind: MemoryKind;

  // Always human-readable (for debugging)
  content: string;

  // Creation time (ms since epoch)
  createdAt: number;

  // Optional last access (for decay / eviction)
  lastAccessedAt?: number;

  // 0.0–1.0 explicit importance
  importance: number;

  // Arbitrary user / system metadata
  metadata: Map<string, string> | Record<string, string>;
}

/**
 * Semantic memory payload - vector embedding for factual knowledge
 */
export interface SemanticPayload extends BaseMemory {
  kind: MemoryKind.Semantic;
  embedding: Float32Array | number[];
}

/**
 * Episodic memory payload - events with timestamps
 */
export interface EpisodicPayload extends BaseMemory {
  kind: MemoryKind.Episodic;
  eventTime: number;
  durationMs: number;
}

/**
 * Working memory payload - short-term context with expiration
 */
export interface WorkingPayload extends BaseMemory {
  kind: MemoryKind.Working;
  size: number;
  expiresAt?: number;
}

/**
 * Procedural memory payload - skills/behaviors with triggers
 */
export interface ProceduralPayload extends BaseMemory {
  kind: MemoryKind.Procedural;
  config: unknown;
  version: string;
  triggers: string[];
}

/**
 * Candidate memory - discriminated union of all memory types
 */
export type Candidate =
  | SemanticPayload
  | EpisodicPayload
  | WorkingPayload
  | ProceduralPayload;

/**
 * Score reason - explanation for why a memory received its score
 */
export type ScoreReason =
  | SemanticSimilarityReason
  | RecencyReason
  | ImportanceReason
  | WorkingPriorityReason
  | ProceduralMatchReason;

export interface SemanticSimilarityReason {
  kind: "semantic";
  score: number;
}

export interface RecencyReason {
  kind: "recency";
  score: number;
}

export interface ImportanceReason {
  kind: "importance";
  score: number;
}

export interface WorkingPriorityReason {
  kind: "working";
}

export interface ProceduralMatchReason {
  kind: "procedural";
  trigger: string;
}

/**
 * Retrieved memory - lightweight output from retrieve()
 */
export interface RetrievedMemory {
  id: string;
  kind: MemoryKind;
  score: number;
  reasons: ScoreReason[];
}

/**
 * Explained memory - detailed output with scoring breakdown
 */
export interface ExplainedMemory {
  id: string;
  kind: MemoryKind;
  score: number;
  reasons: ScoreReason[];
}

/**
 * Retrieval explanation - full output from explain()
 */
export interface RetrievalExplanation {
  considered: number;
  returned: number;
  memories: ExplainedMemory[];
}

/**
 * Retrieve input - parameters for retrieval operation
 */
export interface RetrieveInput {
  queryEmbedding?: Float32Array | number[];
  candidates: Candidate[];
  limit?: number;
  now?: number;
}

/**
 * Scoring weights configuration
 */
export interface Weights {
  semantic: number;
  recency: number;
  workingBoost: number;
  procedural: number;
}

/**
 * Default scoring weights
 * Note: These will move to SDK layer as user-configurable presets in future
 */
export function defaultWeights(): Weights {
  return {
    semantic: 1.0,
    recency: 0.5,
    workingBoost: 1.5,
    procedural: 1.2,
  };
}
