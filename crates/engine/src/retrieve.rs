use std::cmp::Ordering;

use crate::model::MemoryRecord;
use crate::utils::{
    current_time_ms, layer_weight, recency_score, similarity_score,
};

#[derive(Debug, Clone)]
pub struct ScoreBreakdown {
    pub similarity: f32,
    pub recency: f32,
    pub layer_weight: f32,
}

#[derive(Debug, Clone)]
pub struct MemoryCandidate {
    pub record: MemoryRecord,
    pub score: ScoreBreakdown,
    pub confidence: f32,
}

pub struct RetrieveOptions {
    pub top_k: usize,
    pub min_score: f32,
}

/// Main retrieval entry point
pub fn retrieve_memory(
    query: &str,
    candidates: impl Iterator<Item = MemoryRecord>,
    opts: RetrieveOptions,
) -> Vec<MemoryCandidate> {
    let now_ms = current_time_ms();

    let mut scored: Vec<MemoryCandidate> = candidates
        .filter_map(|record| {
            let similarity = similarity_score(query, &record.content);
            let recency = recency_score(now_ms, record.timestamp_ms);
            let layer_weight = layer_weight(&record.layer);

            let total_score =
                similarity * 0.6 + recency * 0.3 + layer_weight * 0.1;

            if total_score < opts.min_score {
                return None;
            }

            Some(MemoryCandidate {
                record,
                score: ScoreBreakdown {
                    similarity,
                    recency,
                    layer_weight,
                },
                confidence: total_score.clamp(0.0, 1.0),
            })
        })
        .collect();

    // Sort by confidence (descending)
    scored.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(Ordering::Equal)
    });

    // Enforce top-k bound
    scored.truncate(opts.top_k);

    scored
}
