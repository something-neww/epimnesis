use crate::types::*;
use engine::{MemoryCandidate, MemoryLayer, MemoryRecord, ScoreBreakdown};

impl TryFrom<MemoryRecordDto> for MemoryRecord {
    type Error = String;

    fn try_from(dto: MemoryRecordDto) -> Result<Self, Self::Error> {
        let layer = match dto.layer.as_str() {
            "working" => MemoryLayer::Working,
            "episodic" => MemoryLayer::Episodic,
            "semantic" => MemoryLayer::Semantic,
            "procedural" => MemoryLayer::Procedural,
            _ => return Err(format!("Unknown memory layer: {}", dto.layer)),
        };

        Ok(MemoryRecord {
            id: dto.id,
            layer,
            content: dto.content,
            timestamp_ms: dto.timestamp_ms,
            source: None,
            metadata: dto.metadata.unwrap_or_default(),
        })
    }
}

impl From<ScoreBreakdown> for ScoreBreakdownDto {
    fn from(s: ScoreBreakdown) -> Self {
        Self {
            similarity: s.similarity as f64,
            recency: s.recency as f64,
            layer_weight: s.layer_weight as f64,
        }
    }
}

impl From<MemoryCandidate> for MemoryCandidateDto {
    fn from(c: MemoryCandidate) -> Self {
        Self {
            record: MemoryRecordDto {
                id: c.record.id,
                layer: format!("{:?}", c.record.layer).to_lowercase(),
                content: c.record.content,
                timestamp_ms: c.record.timestamp_ms,
                metadata: Some(c.record.metadata),
            },
            confidence: c.confidence as f64,
            score: c.score.into(),
        }
    }
}
