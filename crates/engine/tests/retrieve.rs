mod helpers;
use engine::MemoryLayer;
use engine::retrieve::{RetrieveOptions, retrieve_memory};
use helpers::*;

#[test]
fn recent_memory_wins() {
    let old = memory_with_ts(
        MemoryLayer::Semantic,
        "user prefers dark mode",
        now_ms() - 86_400_000, // 1 day ago
    );

    let recent = semantic("user prefers dark mode");
    let recent_id = recent.id.clone();

    let candidates = vec![old, recent].into_iter();

    let out = retrieve_memory(
        "dark mode",
        candidates,
        RetrieveOptions {
            top_k: 1,
            min_score: 0.0,
        },
    );

    assert_eq!(out[0].record.id, recent_id);
}
