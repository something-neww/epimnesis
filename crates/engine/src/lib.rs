// NOTE:
// This crate is a stateless memory reasoning engine.
// All storage, embeddings, orchestration, and frameworks
// must live outside this crate.

pub mod memory;
pub mod scoring;
pub mod weights;

use memory::*;

/// MemoryEngine is intentionally stateless.
///
/// Responsibilities:
/// - score candidate memories
/// - rank them deterministically
/// - explain retrieval decisions
///
/// Non-responsibilities:
/// - storage
/// - embeddings
/// - memory persistence
/// - lifecycle management
///
/// These concerns live in the SDK / application layer.
pub struct MemoryEngine;

impl MemoryEngine {
    pub fn new() -> Self {
        Self
    }

    /// Retrieve ranked memories (lightweight).
    pub fn retrieve(&self, input: RetrieveInput) -> Vec<RetrievedMemory> {
        scoring::retrieve(input)
    }

    /// Retrieve memories with full explanation metadata.
    pub fn explain(&self, input: RetrieveInput) -> RetrievalExplanation {
        scoring::explain(input)
    }
}

impl Default for MemoryEngine {
    fn default() -> Self {
        Self::new()
    }
}
