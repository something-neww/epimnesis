import { MemoryLayer } from "@epimnesis/node";
import type {
  MemoryCandidate,
  MemoryRecord,
  RetrieveOptions,
  ScoreBreakdown,
} from "@epimnesis/node";

export interface MemoryStore {
  create(records: MemoryRecord[]): Promise<void>;
  update(records: MemoryRecord[]): Promise<void>;
  delete(ids: string[]): Promise<void>;
  retrieve(layer: MemoryLayer, limit: number): Promise<MemoryRecord[]>;
}

export type { MemoryRecord, RetrieveOptions, MemoryCandidate, ScoreBreakdown };
export { MemoryLayer };
