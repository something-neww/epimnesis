package vector

import "context"

// Store is the abstraction.
// Other packages import `github.com/epimnesis/core/internal/vector` and use this interface.
type Store interface {
	Insert(ctx context.Context, records []*Model) error
	Search(ctx context.Context, vec []float32, limit int) ([]*Model, error)
	initialize(ctx context.Context) error
}
