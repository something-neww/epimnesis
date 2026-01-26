use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryKind {
    Semantic,   // facts / knowledge
    Episodic,   // events / experiences
    Working,    // short-term context
    Procedural, // skills / behaviors
}

#[derive(Clone, Debug)]
pub struct BaseMemory {
    pub id: String,
    pub kind: MemoryKind,

    /// Always human-readable (debuggable)
    pub content: String,

    /// Creation time (ms since epoch)
    pub created_at: i64,

    /// Optional last access (for decay / eviction)
    pub last_accessed_at: Option<i64>,

    /// 0.0–1.0 explicit importance
    pub importance: f32,

    /// Arbitrary user / system metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct SemanticPayload {
    pub embedding: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct EpisodicPayload {
    /// When the event started
    pub event_time: i64,

    /// Optional duration
    pub duration_ms: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct WorkingPayload {
    /// Token count (or char count)
    pub size: usize,

    /// Hard expiry (optional)
    pub expires_at: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct ProceduralPayload {
    /// Opaque structured config (JSON/YAML/etc)
    pub config: String,

    /// Schema / versioning
    pub version: String,

    /// Optional activation tags
    pub triggers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CandidateMemory {
    pub base: BaseMemory,

    // Optional capabilities
    pub semantic: Option<SemanticPayload>,
    pub episodic: Option<EpisodicPayload>,
    pub working: Option<WorkingPayload>,
    pub procedural: Option<ProceduralPayload>,
}

pub struct RetrieveInput {
    pub query_embedding: Option<Vec<f32>>,

    /// Pre-filtered, authorized, deduped
    pub candidates: Vec<CandidateMemory>,

    pub limit: usize,
    pub now: i64,
}

#[derive(Debug)]
pub enum ScoreReason {
    SemanticSimilarity { score: f32 },
    Recency { score: f32 },
    Importance { score: f32 },
    WorkingPriority,
    ProceduralMatch { trigger: String },
}

pub struct RetrievedMemory {
    pub id: String,
    pub kind: MemoryKind,
    pub score: f32,

    /// Human-readable explanation
    pub reasons: Vec<ScoreReason>,
}

/// Full explanation of a retrieval operation.
///
/// This is a first-class engine output meant for:
/// - debugging
/// - observability
/// - tracing
/// - dashboards
pub struct RetrievalExplanation {
    /// Number of candidate memories considered
    pub considered: usize,

    /// Number of memories returned after ranking + truncation
    pub returned: usize,

    /// Ranked memories with detailed scoring reasons
    pub memories: Vec<ExplainedMemory>,
}

pub struct ExplainedMemory {
    pub id: String,
    pub kind: MemoryKind,
    pub score: f32,
    pub reasons: Vec<ScoreReason>,
}
