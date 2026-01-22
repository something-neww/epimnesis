import test from "ava";

import { retrieve, MemoryLayer } from "../index";

test("retrieve ranks recent semantic memory higher", (t) => {
  const now = Date.now();

  const memories = [
    {
      id: "1",
      layer: MemoryLayer.Semantic,
      content: "user prefers dark mode",
      timestampMs: now - 86400000,
      metadata: {},
    },
    {
      id: "2",
      layer: MemoryLayer.Semantic,
      content: "user prefers dark mode",
      timestampMs: now,
      metadata: {},
    },
  ];

  const out = retrieve("dark mode", memories, { topK: 1, minScore: 0 });
  t.is(out[0].record.id, "2");
});
