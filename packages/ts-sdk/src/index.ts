/**
 * Public SDK API.
 * Breaking changes require a major version bump.
 */

import { JsMemoryEngine } from "@epimnesis/node";
import { RetrieveMapper } from "./ffi";
import type { Candidate } from "./types";

export class Epimnesis {
  #engine: JsMemoryEngine;

  constructor() {
    this.#engine = new JsMemoryEngine();
  }

  retrieve(input: {
    queryEmbedding?: number[];
    candidates: Candidate[];
    limit?: number;
    now?: number;
  }) {
    return this.#engine.retrieve(RetrieveMapper.toJsInput(input));
  }

  recall(input: {
    queryEmbedding?: number[];
    candidates: Candidate[];
    limit?: number;
    now?: number;
  }) {
    return this.#engine.explain(RetrieveMapper.toJsInput(input));
  }
}

export type { ScoreReason, RetrievedMemory, RetrievalExplanation } from "./types";
export type { ScoreReasonKind, MemoryKind } from "@epimnesis/node";
