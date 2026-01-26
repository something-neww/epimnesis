use crate::memory::*;
use crate::weights::default_weights;

/// Rank and score candidate memories.
///
/// ARCHITECTURAL NOTES:
/// - This function is PURE with respect to storage and infrastructure.
/// - All candidates are pre-filtered, authorized, and deduplicated
///   by the TypeScript SDK before being passed in.
/// - This function is responsible ONLY for:
///     - scoring
///     - ranking
///     - explainability
///
/// IMPORTANT:
/// - Scoring weights and decay parameters currently live in Rust
///   as DEFAULT heuristics.
/// - These will move to the TS SDK layer once real-world usage
///   validates which knobs actually matter.
pub fn retrieve(input: RetrieveInput) -> Vec<RetrievedMemory> {
    debug_assert!(input.limit > 0, "RetrieveInput.limit must be > 0");

    explain(input)
        .memories
        .into_iter()
        .map(|m| RetrievedMemory {
            id: m.id,
            kind: m.kind,
            score: m.score,
            reasons: m.reasons,
        })
        .collect()
}

pub fn explain(input: RetrieveInput) -> RetrievalExplanation {
    let weights = default_weights();
    let considered = input.candidates.len();

    let mut scored: Vec<ExplainedMemory> = Vec::with_capacity(considered);

    for candidate in input.candidates {
        let mut score = 0.0;
        let mut reasons: Vec<ScoreReason> = Vec::new();

        // --- invariant: at least one signal must exist ---
        debug_assert!(
            candidate.semantic.is_some()
                || candidate.episodic.is_some()
                || candidate.working.is_some()
                || candidate.procedural.is_some(),
            "CandidateMemory has no scoring signals"
        );

        // Semantic
        if let (Some(q), Some(semantic)) =
            (&input.query_embedding, &candidate.semantic)
        {
            let s = cosine(q, &semantic.embedding);
            if s > 0.0 {
                score += s * weights.semantic;
                reasons.push(ScoreReason::SemanticSimilarity { score: s });
            }
        }

        // Episodic
        if let Some(ep) = &candidate.episodic {
            let r = compute_recency(ep.event_time, input.now);
            if r > 0.0 {
                score += r * weights.recency;
                reasons.push(ScoreReason::Recency { score: r });
            }
        }

        // Working
        if candidate.working.is_some() {
            score += weights.working_boost;
            reasons.push(ScoreReason::WorkingPriority);
        }

        // Procedural
        if let Some(proc_mem) = &candidate.procedural
            && let Some(trigger) = matches_trigger(proc_mem)
        {
            score += weights.procedural;
            reasons.push(ScoreReason::ProceduralMatch { trigger });
        }

        // Engine invariant:
        // If a memory cannot explain *why* it scored,
        // it must not participate in ranking.
        if reasons.is_empty() {
            continue;
        }

        scored.push(ExplainedMemory {
            id: candidate.base.id,
            kind: candidate.base.kind,
            score,
            reasons,
        });
    }

    // Stable, deterministic ordering
    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            // Stable fallback: deterministic ID ordering
            .then_with(|| a.id.cmp(&b.id))
    });

    let returned = scored.len().min(input.limit);
    scored.truncate(input.limit);

    RetrievalExplanation {
        considered,
        returned,
        memories: scored,
    }
}

/// Compute cosine similarity between two vectors.
///
/// Returns a value in [0.0, 1.0].
/// Invalid or mismatched inputs return 0.0.
///
/// NOTE:
/// - This function must NEVER panic.
/// - Engine safety > mathematical purity.
fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

/// Compute a recency score for an episodic memory.
///
/// Returns a value in [0.0, 1.0]:
/// - 1.0 → just happened
/// - 0.0 → very old
///
/// IMPLEMENTATION:
/// - Uses exponential decay with a fixed half-life
/// - Deterministic and monotonic
///
/// FUTURE:
/// - Decay configuration (half-life, curve) will move
///   to the TS SDK layer once validated by real usage.
fn compute_recency(event_time: i64, now: i64) -> f32 {
    if event_time >= now {
        return 1.0;
    }

    let age_ms = (now - event_time) as f32;

    // Half-life: 24 hours
    const HALF_LIFE_MS: f32 = 24.0 * 60.0 * 60.0 * 1000.0;

    // score = 0.5^(age / half-life)
    (0.5_f32).powf(age_ms / HALF_LIFE_MS)
}

/// Determine whether a procedural memory should activate.
///
/// IMPORTANT:
/// - This function MUST remain deterministic.
/// - No external state, no randomness, no I/O.
/// - The TS SDK is responsible for defining what triggers exist.
///
/// CURRENT:
/// - First matching trigger wins (simple heuristic)
///
/// FUTURE:
/// - Context-aware or weighted activation strategies
fn matches_trigger(proc_mem: &ProceduralPayload) -> Option<String> {
    proc_mem.triggers.first().cloned()
}
