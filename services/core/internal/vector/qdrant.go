package vector

import (
	"context"

	"github.com/epimnesis/core/internal/config"
	"github.com/qdrant/go-client/qdrant"
)

type QdrantStore struct {
	client *qdrant.Client
	cfg    *config.Config
}

func newQdrantStore(cfg *config.Config) Store {
	client, _ := qdrant.NewClient(&qdrant.Config{Host: cfg.QdrantURL})
	return &QdrantStore{client: client, cfg: cfg}
}

func (q *QdrantStore) initialize(ctx context.Context) error {
	// Create Collection logic...
	return nil
}

func (q *QdrantStore) Insert(
	ctx context.Context,
	records []*Model,
) error {
	// Qdrant Upsert logic...
	return nil
}

func (q *QdrantStore) Search(
	ctx context.Context,
	vec []float32, limit int,
) ([]*Model, error) {
	// Qdrant Search logic...
	return nil, nil
}
