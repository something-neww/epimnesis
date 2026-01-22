import {
  createEpimnesis,
  MemoryLayer,
  type MemoryRecord,
  type MemoryStore,
} from "@epimnesis/core";

export function createInMemoryStore(): MemoryStore {
  const records: MemoryRecord[] = [];

  return {
    async create(rs) {
      records.push(...rs);
    },

    async update(_rs) {
      /* noop for now */
    },

    async delete(ids) {
      ids.forEach((id) => {
        const i = records.findIndex((r) => r.id === id);
        if (i >= 0) records.splice(i, 1);
      });
    },

    async retrieve(layer, limit) {
      return records.filter((r) => r.layer === layer).slice(0, limit);
    },
  };
}

const store = createInMemoryStore();
const epi = createEpimnesis({ store });

await store.create([
  {
    id: "1",
    layer: MemoryLayer.Semantic,
    content: "user prefers dark mode",
    timestampMs: Date.now(),
  },
]);

const out = await epi.retrieve("dark mode", [MemoryLayer.Semantic]);
console.log(out);
