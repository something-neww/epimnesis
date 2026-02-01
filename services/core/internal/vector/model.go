package vector

import (
	"time"

	"github.com/pgvector/pgvector-go"
)

type Model struct {
	ID        string          `json:"id"`
	Vector    pgvector.Vector `json:"vector"`
	Metadata  map[string]any  `json:"metadata"`
	CreatedAt time.Time       `json:"created_at"`
}
