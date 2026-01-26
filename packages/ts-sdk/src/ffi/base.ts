import type { JsBaseMemory } from "@epimnesis/node";
import type { Candidate } from "../types";

/**
 * BaseMemory mapper
 * SDK (camelCase, ergonomic) -> NAPI DTO (boundary truth)
 */
export class BaseMemoryMapper {
  static toJs(memory: Candidate): JsBaseMemory {
    return {
      id: memory.id,
      kind: memory.kind,
      content: memory.content,
      createdAt: memory.createdAt,
      lastAccessedAt: memory.lastAccessedAt,
      importance: memory.importance,
      metadata: memory.metadata,
    };
  }
}
