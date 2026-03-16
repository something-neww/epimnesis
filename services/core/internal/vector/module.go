// Package vector
package vector

import (
	"context"

	"github.com/epimnesis/core/internal/config"
	"go.uber.org/fx"
)

// Module exports the Fx dependencies for this package
var Module = fx.Options(
	// 1. Provide the factory function
	fx.Provide(newVectorStore),
)

// newVectorStore is the Factory.
// It is the ONLY function exported from this package that returns the Store interface.
func newVectorStore(cfg config.Config) (Store, error) {
	var store Store

	switch cfg.VectorStore {
	case "qdrant":
		store = newQdrantStore(cfg)
	default:
		store = newPostgresStore(cfg)
	}

	// Initialize (Migration) immediately
	if err := store.initialize(context.Background()); err != nil {
		return nil, err
	}

	return store, nil
}
