package config

import (
	"fmt"
	"os"
	"strconv"
)

// Config holds configuration for vector store backends
type Config struct {
	// VectorStore selects which backend to use: "pgvector" or "qdrant"
	VectorStore string

	// EmbeddingDimension is the size of the embedding vector (default: 1536 for OpenAI ada-002)
	EmbeddingDimension int

	// DatabaseURL is the PostgreSQL connection string (required for pgvector backend)
	DatabaseURL string

	// QdrantHost is the Qdrant server hostname (default: localhost)
	QdrantHost string

	// QdrantPort is the Qdrant server port (default: 6334)
	QdrantPort string

	// CollectionName is the Qdrant collection name (default: memories)
	CollectionName string
}

// LoadConfig reads environment variables and returns Config struct
func LoadConfig() (*Config, error) {
	cfg := &Config{}

	// VectorStore: default to "pgvector" if not set
	cfg.VectorStore = os.Getenv("VECTOR_STORE")
	if cfg.VectorStore == "" {
		cfg.VectorStore = "pgvector"
	}

	// Validate VectorStore is either "pgvector" or "qdrant"
	if cfg.VectorStore != "pgvector" && cfg.VectorStore != "qdrant" {
		return nil, fmt.Errorf("Invalid VECTOR_STORE value: %s (must be 'qdrant' or 'pgvector')", cfg.VectorStore)
	}

	// EmbeddingDimension: default to 1536 if not set
	embDimStr := os.Getenv("EMBEDDING_DIMENSION")
	if embDimStr == "" {
		cfg.EmbeddingDimension = 1536
	} else {
		embDim, err := strconv.Atoi(embDimStr)
		if err != nil {
			return nil, fmt.Errorf("Invalid EMBEDDING_DIMENSION value: %s (must be a valid integer)", embDimStr)
		}
		cfg.EmbeddingDimension = embDim
	}

	// DatabaseURL: required (no default, will be validated by adapter)
	cfg.DatabaseURL = os.Getenv("DATABASE_URL")

	// QdrantHost: default to "localhost"
	cfg.QdrantHost = os.Getenv("QDRANT_HOST")
	if cfg.QdrantHost == "" {
		cfg.QdrantHost = "localhost"
	}

	// QdrantPort: default to "6334"
	cfg.QdrantPort = os.Getenv("QDRANT_PORT")
	if cfg.QdrantPort == "" {
		cfg.QdrantPort = "6334"
	}

	// CollectionName: default to "memories"
	cfg.CollectionName = os.Getenv("QDRANT_COLLECTION_NAME")
	if cfg.CollectionName == "" {
		cfg.CollectionName = "memories"
	}

	return cfg, nil
}
