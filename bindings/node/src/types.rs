use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct JsBaseMemory {
    pub id: String,
    pub kind: String,

    pub content: String,
    pub created_at: i64,
    pub last_accessed_at: i64,
    pub importance: f64,
    pub metadata: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct JsEpisodicPayload {
    pub event_time: i64,
    pub duration_ms: Option<i64>,
}

#[napi(object)]
pub struct JsSemanticPayload {
    pub embedding: Vec<f64>,
}

#[napi(object)]
pub struct JsWorkingPayload {
    pub expires_at: Option<i64>,
    pub size: Option<u32>,
}

#[napi(object)]
pub struct JsProceduralPayload {
    pub triggers: Vec<String>,
    pub config: Option<String>,
    pub version: Option<String>,
}

#[napi(object)]
pub struct JsCandidateMemory {
    pub base: JsBaseMemory,

    pub semantic: Option<JsSemanticPayload>,
    pub episodic: Option<JsEpisodicPayload>,
    pub working: Option<JsWorkingPayload>,
    pub procedural: Option<JsProceduralPayload>,
}

#[napi(object)]
pub struct JsRetrieveInput {
    pub query_embedding: Option<Vec<f64>>,
    pub candidates: Vec<JsCandidateMemory>,
    pub limit: u32,
    pub now: i64,
}

#[napi(object)]
pub struct JsScoreReason {
    pub kind: String,
    pub score: Option<f64>,
    pub trigger: Option<String>,
}

#[napi(object)]
pub struct JsRetrievedMemory {
    pub id: String,
    pub kind: String,
    pub score: f64,
    pub reasons: Vec<JsScoreReason>,
}

#[napi(object)]
pub struct JsRetrievalExplanation {
    pub considered: u32,
    pub returned: u32,
    pub memories: Vec<JsRetrievedMemory>,
}
