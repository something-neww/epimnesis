# @epimnesis/core

The core TypeScript SDK for Epimnesis.

## MemoryStore Interface

The `MemoryStore` interface defines the contract for storage adapters.

### Standard Operations

Use these for standard creation and updates:

```typescript
// Create records
await store.create([record]);

// Update records
await store.update([record]);

// Delete by ID
await store.delete(["id1", "id2"]);
```

### Batch Operations

For efficient bulk ingestion or phase 1 workflows, use the dedicated batch methods. Adapters are encouraged to optimize these for high-throughput:

```typescript
// Batch create multiple records
await store.createBatch([record1, record2, ...]);

// Batch update multiple records
await store.updateBatch([record1, record2, ...]);
```

## In-Memory Store

The SDK provides a built-in in-memory store for development and testing:

```typescript
import { createInMemoryStore } from "@epimnesis/core";

const store = createInMemoryStore();
```

## Development

- Install dependencies:

```bash
pnpm install
```

- Run the unit tests:

```bash
pnpm run test
```

- Build the library:

```bash
pnpm run build
```