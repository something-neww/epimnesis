import { describe, test, expect } from "vitest";

import {
  MemoryEngine,
  MemoryKind,
  type Candidate,
  type RetrieveInput,
} from "../engine/index.js";

function semantic(id: string, embedding: number[]): Candidate {
  return {
    id,
    kind: MemoryKind.Semantic,
    content: "",
    createdAt: 0,
    importance: 0.0,
    metadata: {},
    embedding,
  };
}

describe("MemoryEngine", () => {
  test("semantic similarity ranks higher", () => {
    const engine = new MemoryEngine();
    const q = [1.0, 0.0];

    const a = semantic("a", [1.0, 0.0]);
    const b = semantic("b", [0.0, 1.0]);

    const input: RetrieveInput = {
      queryEmbedding: q,
      candidates: [b, a],
      limit: 2,
      now: 0,
    };

    const result = engine.explain(input);

    expect(result.memories[0].id).toBe("a");
  });

  test("retrieves correct number of results", () => {
    const engine = new MemoryEngine();
    const q = [1.0, 0.0];

    const a = semantic("a", [1.0, 0.0]);
    const b = semantic("b", [0.5, 0.5]);
    const c = semantic("c", [0.0, 1.0]);

    const input: RetrieveInput = {
      queryEmbedding: q,
      candidates: [b, c, a],
      limit: 2,
      now: 0,
    };

    const result = engine.retrieve(input);

    expect(result).toHaveLength(2);
    expect(result[0].id).toBe("a");
  });

  test("handles empty vectors safely", () => {
    const engine = new MemoryEngine();

    const a = semantic("a", []);
    const b = semantic("b", [1.0, 0.0]);

    const input: RetrieveInput = {
      queryEmbedding: [1.0, 0.0],
      candidates: [a, b],
      limit: 10,
      now: 0,
    };

    const result = engine.retrieve(input);

    expect(result[0].id).toBe("b");
    expect(result[0].score).toBeGreaterThan(0);
  });

  test("excludes candidates with no scoring signals", () => {
    const engine = new MemoryEngine();

    const noSignal = {
      id: "no-signal",
      kind: MemoryKind.Semantic,
      content: "",
      createdAt: 0,
      importance: 0.0,
      metadata: {},
    } as Candidate;

    const input: RetrieveInput = {
      candidates: [noSignal],
      limit: 10,
      now: 0,
    };

    const result = engine.retrieve(input);

    expect(result).toHaveLength(0);
  });
});
