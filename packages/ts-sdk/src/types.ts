import type {
  JsBaseMemory,
  MemoryKind,
  JsSemanticPayload,
  JsWorkingPayload,
  JsEpisodicPayload,
  JsProceduralPayload,
  JsRetrievedMemory,
  JsRetrievalExplanation,
  JsScoreReason,
} from "@epimnesis/node";

type BaseMemory = JsBaseMemory;

export interface SemanticPayload extends BaseMemory {
  kind: MemoryKind.Semantic;
  embedding: JsSemanticPayload["embedding"];
}

export interface WorkingPayload extends BaseMemory {
  kind: MemoryKind.Working;
  expiresAt: JsWorkingPayload["expiresAt"];
  size: JsWorkingPayload["size"];
}

export interface EpisodicPayload extends BaseMemory {
  kind: MemoryKind.Episodic;
  durationMs: JsEpisodicPayload["durationMs"];
  eventTime: JsEpisodicPayload["eventTime"];
}

export interface ProceduralPayload extends BaseMemory {
  kind: MemoryKind.Procedural;
  triggers: JsProceduralPayload["triggers"];
  config: JsProceduralPayload["config"];
  version: JsProceduralPayload["version"];
}

export type Candidate = SemanticPayload | WorkingPayload | EpisodicPayload | ProceduralPayload;

export type RetrievedMemory = JsRetrievedMemory;
export type RetrievalExplanation = JsRetrievalExplanation;
export type ScoreReason = JsScoreReason;
