use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::MemoryLayer;

pub fn similarity_score(query: &str, content: &str) -> f32 {
    if query.is_empty() || content.is_empty() {
        return 0.0;
    }

    let content_lower = content.to_lowercase();
    let query_tokens: Vec<String> =
        query.split_whitespace().map(|t| t.to_lowercase()).collect();

    let matches = query_tokens
        .iter()
        .filter(|token| content_lower.contains(token.as_str()))
        .count();

    matches as f32 / query_tokens.len() as f32
}

pub fn recency_score(now_ms: i64, timestamp_ms: i64) -> f32 {
    let age_ms = (now_ms - timestamp_ms).max(0) as f32;

    // Half-life decay: newer = higher score
    let half_life_ms = 1000.0 * 60.0 * 60.0 * 24.0; // 1 day
    (-age_ms / half_life_ms).exp()
}

pub fn layer_weight(layer: &MemoryLayer) -> f32 {
    match layer {
        MemoryLayer::Working => 1.0,
        MemoryLayer::Episodic => 0.7,
        MemoryLayer::Semantic => 0.5,
        MemoryLayer::Procedural => 0.3,
    }
}

pub fn current_time_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
