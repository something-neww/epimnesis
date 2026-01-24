import {
  createEpimnesis,
  createInMemoryStore,
  MemoryLayer,
} from "@epimnesis/core";

const store = createInMemoryStore();
const epi = createEpimnesis({ store });

/** Create */
await store.create([
  {
    id: "0",
    layer: MemoryLayer.Semantic,
    content: "user prefers dark mode",
    timestampMs: Date.now(),
  },
  {
    id: "1",
    layer: MemoryLayer.Semantic,
    content: "user loves patting cats",
    timestampMs: Date.now(),
  },
]);

/** Batch Create */
await store.createBatch([
  {
    id: "0",
    layer: MemoryLayer.Semantic,
    content: "user prefers dark mode",
    timestampMs: Date.now(),
  },
  {
    id: "1",
    layer: MemoryLayer.Semantic,
    content: "user loves patting cats",
    timestampMs: Date.now(),
  },
]);

const out = await epi.retrieve("dark mode", [MemoryLayer.Semantic]);
console.log(`[EpiRetrieve]`, out);
