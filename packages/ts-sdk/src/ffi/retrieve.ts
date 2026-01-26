import type { JsRetrieveInput, JsRetrievedMemory, JsRetrievalExplanation } from "@epimnesis/node";

import type { Candidate, RetrievalExplanation, RetrievedMemory } from "../types";
import { CandidateMapper } from "./candidate";

/**
 * Retrieval mappers
 */
export class RetrieveMapper {
  static toJsInput(input: {
    queryEmbedding?: number[];
    candidates: Candidate[];
    limit?: number;
    now?: number;
  }): JsRetrieveInput {
    return {
      queryEmbedding: input.queryEmbedding,
      candidates: input.candidates.map(CandidateMapper.toJs),
      limit: input.limit ?? 5,
      now: input.now ?? Date.now(),
    };
  }

  // NOTE: Output types are re-exported directly from NAPI
  // JsRetrievedMemory and JsRetrievalExplanation are treated as read-only truth
  static fromJsRetrievedMemory(m: JsRetrievedMemory): RetrievedMemory {
    return m;
  }

  static fromJsExplanation(e: JsRetrievalExplanation): RetrievalExplanation {
    return e;
  }
}
