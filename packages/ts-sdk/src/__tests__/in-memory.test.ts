import { describe, it, expect, vi } from "vitest";

import { MemoryLayer, createInMemoryStore } from "../index";
import type { MemoryRecord } from "../types";

describe("MemoryStore Batch Operations", () => {
  it("should support createBatch", async () => {
    const store = createInMemoryStore();
    const records: MemoryRecord[] = [
      {
        id: "1",
        layer: MemoryLayer.Working,
        content: "A",
        timestampMs: 100,
        metadata: { source: "test" },
      },
      {
        id: "2",
        layer: MemoryLayer.Working,
        content: "B",
        timestampMs: 101,
        metadata: { source: "test" },
      },
    ];

    await store.createBatch(records);

    const stored = await store.retrieve(MemoryLayer.Working, 10);
    expect(stored).toHaveLength(2);
    expect(stored[0].id).toBe("1");
    expect(stored[1].id).toBe("2");
  });

  it("should support updateBatch", async () => {
    const store = createInMemoryStore();
    const records: MemoryRecord[] = [
      {
        id: "1",
        layer: MemoryLayer.Working,
        content: "A",
        timestampMs: 100,
        metadata: { source: "test" },
      },
    ];
    await expect(store.updateBatch(records)).resolves.not.toThrow();
  });
});
