/**
 * MemoryEngine - stateless scoring and ranking engine
 * Ported from Rust engine, pure TypeScript implementation
 */

import {
  cosineSimilarity,
  computeRecency,
  matchesTrigger,
  getPayload,
} from "./scoring.js";
import type {
  RetrievedMemory,
  RetrievalExplanation,
  ExplainedMemory,
  RetrieveInput,
  ScoreReason,
} from "./types.js";
import { defaultWeights, MemoryKind } from "./types.js";
export { MemoryKind, type Candidate } from "./types.js";

export class MemoryEngine {
  constructor() {}

  retrieve(input: RetrieveInput): RetrievedMemory[] {
    const result = this.explain(input);
    return result.memories.map(
      (m): RetrievedMemory => ({
        id: m.id,
        kind: m.kind,
        score: m.score,
        reasons: m.reasons,
      }),
    );
  }

  explain(input: RetrieveInput): RetrievalExplanation {
    const weights = defaultWeights();
    const { candidates, queryEmbedding, limit = 5, now = Date.now() } = input;

    const scored: ExplainedMemory[] = [];

    for (const candidate of candidates) {
      const reasons: ScoreReason[] = [];
      let score = 0.0;

      const semantic = getPayload(candidate, MemoryKind.Semantic);
      const episodic = getPayload(candidate, MemoryKind.Episodic);
      const working = getPayload(candidate, MemoryKind.Working);
      const procedural = getPayload(candidate, MemoryKind.Procedural);

      if (semantic && queryEmbedding) {
        const s = cosineSimilarity(queryEmbedding, semantic.embedding);
        if (s > 0.0) {
          score += s * weights.semantic;
          reasons.push({ kind: "semantic", score: s });
        }
      }

      if (episodic) {
        const r = computeRecency(episodic.eventTime, now);
        if (r > 0.0) {
          score += r * weights.recency;
          reasons.push({ kind: "recency", score: r });
        }
      }

      if (working) {
        score += weights.workingBoost;
        reasons.push({ kind: "working" });
      }

      if (procedural) {
        const trigger = matchesTrigger(procedural);
        if (trigger) {
          score += weights.procedural;
          reasons.push({ kind: "procedural", trigger });
        }
      }

      if (reasons.length === 0) {
        continue;
      }

      scored.push({
        id: candidate.id,
        kind: candidate.kind,
        score,
        reasons,
      });
    }

    scored.sort((a, b) => {
      const scoreDiff = b.score - a.score;
      if (scoreDiff !== 0) {
        return scoreDiff;
      }
      return a.id.localeCompare(b.id);
    });

    const returned = Math.min(scored.length, limit);
    const truncated = scored.slice(0, limit);

    return {
      considered: candidates.length,
      returned,
      memories: truncated,
    };
  }
}

export function createEngine(): MemoryEngine {
  return new MemoryEngine();
}
