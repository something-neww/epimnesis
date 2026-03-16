package config

import "os"

type Config struct {
	// "pgvector" or "qdrant". Defaults to pgvector.
	VectorStore string `env:"VECTOR_STORE" envDefault:"pgvector"`

	// Postgres Config (Only used if pgvector)
	DatabaseURL string `env:"DATABASE_URL"`

	// Qdrant Config (Only used if qdrant)
	QdrantURL    string `env:"QDRANT_URL"`
	QdrantAPIKey string `env:"QDRANT_API_KEY"`

	// Vector Config
	EmbeddingDimension int `env:"EMBEDDING_DIMENSION" envDefault:"1536"`
}

func Load() Config {
	// Use your preferred env loader (e.g., envconfig, viper)
	return Config{
		VectorStore:        getEnv("VECTOR_STORE", "pgvector"),
		DatabaseURL:        getEnv("DATABASE_URL", ""),
		QdrantURL:          getEnv("QDRANT_URL", "http://localhost:6334"),
		QdrantAPIKey:       getEnv("QDRANT_API_KEY", ""),
		EmbeddingDimension: 1536,
	}
}

func getEnv(key, defaultVal string) string {
	if val := os.Getenv(key); val != "" {
		return val
	}
	return defaultVal
}
