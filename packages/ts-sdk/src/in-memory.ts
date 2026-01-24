import type { MemoryRecord, MemoryStore } from "./types";

export function createInMemoryStore(): MemoryStore {
  const records: MemoryRecord[] = [];

  return {
    async create(rs) {
      console.log(`[InMemoryStore] create received for ${rs.length} records`);
      records.push(...rs);
    },

    async update(rs) {
      console.log(`[InMemoryStore] update received for ${rs.length} records`);
    },

    async createBatch(rs) {
      console.log(
        `[InMemoryStore] Batch create received for ${rs.length} records`,
      );
      records.push(...rs);
    },

    async updateBatch(rs) {
      console.log(
        `[InMemoryStore] Batch update received for ${rs.length} records`,
      );
    },

    async delete(ids) {
      console.log(`[InMemoryStore] Delete received for ${ids.length} records`);
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
