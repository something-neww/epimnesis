/**
 * Public SDK API.
 * Breaking changes require a major version bump.
 */

import { retrieve as engineRetrieve, MemoryLayer } from "@epimnesis/node";

import type { MemoryRecord, MemoryStore } from "./types";

export function createEpimnesis(opts: { store: MemoryStore }) {
  return {
    async retrieve(query: string, layers: MemoryLayer[]) {
      const candidates: MemoryRecord[] = [];

      for (const layer of layers) {
        const layerRecords = await opts.store.retrieve(layer, 50);
        candidates.push(...layerRecords);
      }

      return engineRetrieve(query, candidates, {
        topK: 10,
        minScore: 0,
      });
    },
  };
}

export * from "./types";
