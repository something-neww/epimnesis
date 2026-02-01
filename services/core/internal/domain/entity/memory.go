package entity

type Memory struct {
	ID        string         `json:"id"`
	SessionID string         `json:"session_id"`
	Content   string         `json:"content"`
	Embedding []float32      `json:"embedding"`
	Metadata  map[string]any `json:"metadata"`
}
