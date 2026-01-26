use engine::memory::*;
use engine::scoring::*;

fn base(id: &str) -> BaseMemory {
    BaseMemory {
        id: id.to_string(),
        kind: MemoryKind::Semantic,

        // Engine does NOT reason over content directly
        content: String::new(),

        // Deterministic timestamps for tests
        created_at: 0,
        last_accessed_at: Some(0),

        // Neutral importance (engine tie-breakers may use this later)
        importance: 0.0,

        // Empty metadata — engine ignores this
        metadata: Default::default(),
    }
}

fn semantic(id: &str, embedding: Vec<f32>) -> CandidateMemory {
    CandidateMemory {
        base: base(id),
        semantic: Some(SemanticPayload { embedding }),
        episodic: None,
        working: None,
        procedural: None,
    }
}

#[test]
fn semantic_similarity_ranks_higher() {
    let q = vec![1.0, 0.0];

    let a = semantic("a", vec![1.0, 0.0]);
    let b = semantic("b", vec![0.0, 1.0]);

    let out = explain(RetrieveInput {
        query_embedding: Some(q),
        candidates: vec![b, a],
        limit: 2,
        now: 0,
    });

    assert_eq!(out.memories[0].id, "a");
}
