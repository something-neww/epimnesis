use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use engine::{MemoryLayer, MemoryRecord};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> String {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed).to_string()
}

pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Generic memory (explicit layer)
pub fn memory(layer: MemoryLayer, content: &str) -> MemoryRecord {
    MemoryRecord {
        id: next_id(),
        layer,
        content: content.to_string(),
        timestamp_ms: now_ms(),
        source: None,
        metadata: HashMap::new(),
    }
}

/// Semantic memory
pub fn semantic(content: &str) -> MemoryRecord {
    memory(MemoryLayer::Semantic, content)
}

/// Episodic memory
pub fn episodic(content: &str) -> MemoryRecord {
    memory(MemoryLayer::Episodic, content)
}

/// Working memory
pub fn working(content: &str) -> MemoryRecord {
    memory(MemoryLayer::Working, content)
}

/// Procedural memory
pub fn procedural(content: &str) -> MemoryRecord {
    memory(MemoryLayer::Procedural, content)
}

/// Memory with explicit timestamp (for recency tests)
pub fn memory_with_ts(
    layer: MemoryLayer,
    content: &str,
    timestamp_ms: i64,
) -> MemoryRecord {
    MemoryRecord {
        id: next_id(),
        layer,
        content: content.to_string(),
        timestamp_ms,
        source: None,
        metadata: HashMap::new(),
    }
}
