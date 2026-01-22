use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryLayer {
    Working,
    Episodic,
    Semantic,
    Procedural,
}

#[derive(Clone, Debug)]
pub struct MemoryRecord {
    pub id: String,
    pub layer: MemoryLayer,
    pub content: String,
    pub timestamp_ms: i64,
    pub source: Option<String>,
    pub metadata: HashMap<String, String>,
}
