package vector

import (
	"context"
	"database/sql"

	"github.com/epimnesis/core/internal/config"
	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
)

type PostgresStore struct {
	db  *bun.DB
	cfg *config.Config
}

// NewPostgresStore creates the implementation.
// It is NOT exported (lowercase) so it can't be used outside this package directly.
func newPostgresStore(cfg *config.Config) Store {
	sqldb := sql.OpenDB(pgdriver.NewConnector(pgdriver.WithDSN(cfg.DatabaseURL)))
	return &PostgresStore{
		db:  bun.NewDB(sqldb, pgdialect.New()),
		cfg: cfg,
	}
}

func (p *PostgresStore) initialize(ctx context.Context) error {
	// Run Migrations...
	return nil
}

func (p *PostgresStore) Insert(
	ctx context.Context,
	records []*Model,
) error {
	_, err := p.db.NewInsert().Model(&records).Exec(ctx)
	return err
}

func (p *PostgresStore) Search(
	ctx context.Context,
	vec []float32, limit int,
) ([]*Model, error) {
	var results []*Model
	err := p.db.NewSelect().Model(&results).OrderExpr("embedding <=> ?", vec).Limit(limit).Scan(ctx)
	return results, err
}
