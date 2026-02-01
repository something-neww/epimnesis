package repository

import (
	"context"
	"github.com/epimnesis/core/internal/domain/entity"
)

// MemoryRepository defines the interface for memory persistence operations.
// This is a domain interface following the Clean Architecture pattern.
type MemoryRepository interface {
	// Store stores multiple memory entities to the repository.
	// Implementations are responsible for batching and error handling.
	Store(ctx context.Context, memories []*entity.Memory) error

	// Search searches for memories based on vector similarity.
	// Returns the most similar memories up to the specified limit.
	// The vector parameter is a semantic embedding vector for comparison.
	Search(ctx context.Context, vector []float32, limit int) ([]*entity.Memory, error)
}
