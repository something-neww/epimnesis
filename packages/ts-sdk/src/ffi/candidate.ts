import {
  MemoryKind,
  type JsCandidateMemory,
  type JsSemanticPayload,
  type JsEpisodicPayload,
  type JsWorkingPayload,
  type JsProceduralPayload,
} from "@epimnesis/node";

import type { Candidate } from "../types";
import { BaseMemoryMapper } from "./base";

/**
 * CandidateMemory mapper
 * Discriminated union -> NAPI shape
 */
export class CandidateMapper {
  static toJs(candidate: Candidate): JsCandidateMemory {
    const base = BaseMemoryMapper.toJs(candidate);

    switch (candidate.kind) {
      case MemoryKind.Semantic:
        return {
          base,
          semantic: {
            embedding: candidate.embedding,
          } satisfies JsSemanticPayload,
        };

      case MemoryKind.Episodic:
        return {
          base,
          episodic: {
            eventTime: candidate.eventTime,
            durationMs: candidate.durationMs,
          } satisfies JsEpisodicPayload,
        };

      case MemoryKind.Working:
        return {
          base,
          working: {
            expiresAt: candidate.expiresAt,
            size: candidate.size,
          } satisfies JsWorkingPayload,
        };

      case MemoryKind.Procedural:
        return {
          base,
          procedural: {
            triggers: candidate.triggers,
            config: candidate.config,
            version: candidate.version,
          } satisfies JsProceduralPayload,
        };

      default: {
        const _exhaustive: never = candidate;
        throw new Error(`Unknown memory kind: ${_exhaustive}`);
      }
    }
  }
}
