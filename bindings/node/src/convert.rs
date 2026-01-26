use crate::types::*;

use engine::memory::{
    BaseMemory, CandidateMemory, EpisodicPayload, ExplainedMemory, MemoryKind,
    ProceduralPayload, RetrievalExplanation, RetrieveInput, RetrievedMemory,
    ScoreReason, SemanticPayload, WorkingPayload,
};

fn parse_memory_kind(kind: &str) -> MemoryKind {
    match kind {
        "semantic" => MemoryKind::Semantic,
        "episodic" => MemoryKind::Episodic,
        "working" => MemoryKind::Working,
        "procedural" => MemoryKind::Procedural,
        _ => MemoryKind::Semantic, // safe default
    }
}

impl From<JsBaseMemory> for BaseMemory {
    fn from(js: JsBaseMemory) -> Self {
        BaseMemory {
            id: js.id,
            kind: parse_memory_kind(&js.kind),
            content: js.content,
            created_at: js.created_at,
            last_accessed_at: Some(js.last_accessed_at),
            importance: js.importance as f32,
            metadata: js.metadata.unwrap_or_default(),
        }
    }
}

impl From<JsSemanticPayload> for SemanticPayload {
    fn from(js: JsSemanticPayload) -> Self {
        Self {
            embedding: js.embedding.into_iter().map(|v| v as f32).collect(),
        }
    }
}

impl From<JsEpisodicPayload> for EpisodicPayload {
    fn from(js: JsEpisodicPayload) -> Self {
        Self {
            event_time: js.event_time,
            duration_ms: Some(js.duration_ms.unwrap_or(0)),
        }
    }
}

impl From<JsWorkingPayload> for WorkingPayload {
    fn from(js: JsWorkingPayload) -> Self {
        Self {
            expires_at: js.expires_at,
            size: js.size.unwrap_or(0) as usize,
        }
    }
}

impl From<JsProceduralPayload> for ProceduralPayload {
    fn from(js: JsProceduralPayload) -> Self {
        Self {
            config: js.config.unwrap_or_default(),
            version: js.version.unwrap_or_else(|| "v1".to_string()),
            triggers: js.triggers,
        }
    }
}

impl From<JsCandidateMemory> for CandidateMemory {
    fn from(js: JsCandidateMemory) -> Self {
        CandidateMemory {
            base: js.base.into(),
            semantic: js.semantic.map(Into::into),
            episodic: js.episodic.map(Into::into),
            working: js.working.map(Into::into),
            procedural: js.procedural.map(Into::into),
        }
    }
}

impl From<JsRetrieveInput> for RetrieveInput {
    fn from(js: JsRetrieveInput) -> Self {
        RetrieveInput {
            query_embedding: js
                .query_embedding
                .map(|v| v.into_iter().map(|x| x as f32).collect()),
            candidates: js.candidates.into_iter().map(Into::into).collect(),
            limit: js.limit as usize,
            now: js.now,
        }
    }
}

impl From<ScoreReason> for JsScoreReason {
    fn from(r: ScoreReason) -> Self {
        match r {
            ScoreReason::SemanticSimilarity { score } => Self {
                kind: "semantic".into(),
                score: Some(score as f64),
                trigger: None,
            },
            ScoreReason::Recency { score } => Self {
                kind: "recency".into(),
                score: Some(score as f64),
                trigger: None,
            },
            ScoreReason::WorkingPriority => Self {
                kind: "working".into(),
                score: None,
                trigger: None,
            },
            ScoreReason::ProceduralMatch { trigger } => Self {
                kind: "procedural".into(),
                score: None,
                trigger: Some(trigger),
            },
            ScoreReason::Importance { score } => Self {
                kind: "importance".into(),
                score: Some(score as f64),
                trigger: None,
            },
        }
    }
}

impl From<RetrievedMemory> for JsRetrievedMemory {
    fn from(m: RetrievedMemory) -> Self {
        Self {
            id: m.id,
            kind: format!("{:?}", m.kind).to_lowercase(),
            score: m.score as f64,
            reasons: m.reasons.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ExplainedMemory> for JsRetrievedMemory {
    fn from(m: ExplainedMemory) -> Self {
        Self {
            id: m.id,
            kind: format!("{:?}", m.kind).to_lowercase(),
            score: m.score as f64,
            reasons: m.reasons.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<RetrievalExplanation> for JsRetrievalExplanation {
    fn from(e: RetrievalExplanation) -> Self {
        Self {
            considered: e.considered as u32,
            returned: e.returned as u32,
            memories: e.memories.into_iter().map(Into::into).collect(),
        }
    }
}
