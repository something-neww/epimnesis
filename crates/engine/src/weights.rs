/// Internal scoring weights used by the Rust engine.
///
/// IMPORTANT ARCHITECTURE NOTE:
/// -----------------------------
/// These weights are intentionally defined *inside* the Rust engine
/// for the initial versions (v0.x) to keep the public API minimal
/// and avoid premature configuration.
///
/// The long-term plan is:
/// - The TypeScript SDK will own user-facing configuration
/// - The SDK will pass weights (or a strategy preset) into the engine
/// - These values will become *defaults*, not hardcoded behavior
///
/// Until real-world usage proves which knobs matter, these defaults
/// serve as a stable, opinionated baseline.
pub struct Weights {
    /// Weight applied to semantic (vector) similarity
    pub semantic: f32,

    /// Weight applied to episodic recency scoring
    pub recency: f32,

    /// Fixed boost for working memory (short-term context)
    pub working_boost: f32,

    /// Fixed boost for procedural memory activation
    pub procedural: f32,
}

/// Default scoring weights used by the engine.
///
/// NOTE:
/// These values are *not* meant to be tuned here long-term.
/// They exist to make the engine usable without configuration.
///
/// TODO (post-v0):
/// - Move weight configuration to the TS SDK
/// - Allow presets (e.g. "chat", "agent", "research")
/// - Pass resolved weights into the engine via RetrieveInput
pub fn default_weights() -> Weights {
    Weights {
        semantic: 1.0,
        recency: 0.5,
        working_boost: 1.5,
        procedural: 1.2,
    }
}
