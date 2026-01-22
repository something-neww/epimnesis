use napi_derive::napi;
use std::collections::HashMap;

// ---------- INPUT DTOs ----------

#[napi(object, js_name = "MemoryRecord")]
pub struct MemoryRecordDto {
    pub id: String,
    pub layer: String, // JS-friendly
    pub content: String,
    pub timestamp_ms: i64,
    pub metadata: Option<HashMap<String, String>>,
}

#[napi(object, js_name = "RetrieveOptions")]
pub struct RetrieveOptionsDto {
    pub top_k: u32,
    pub min_score: f64,
}

// ---------- OUTPUT DTOs ----------

#[napi(object, js_name = "ScoreBreakdown")]
pub struct ScoreBreakdownDto {
    pub similarity: f64,
    pub recency: f64,
    pub layer_weight: f64,
}

#[napi(object, js_name = "MemoryCandidate")]
pub struct MemoryCandidateDto {
    pub record: MemoryRecordDto,
    pub confidence: f64,
    pub score: ScoreBreakdownDto,
}
