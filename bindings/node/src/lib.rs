// NOTE:
// This is the stable Node FFI contract for Epimnesis.
// Breaking changes require a major version bump.

use engine::retrieve_memory;
use napi_derive::napi;

use crate::types::*;

mod convert;
mod types;

#[napi]
pub fn retrieve(
    query: String,
    candidates: Vec<MemoryRecordDto>,
    opts: RetrieveOptionsDto,
) -> napi::Result<Vec<MemoryCandidateDto>> {
    let q = query.as_str();
    let engine_candidates = candidates
        .into_iter()
        .map(|dto| dto.try_into())
        .collect::<Result<Vec<_>, _>>()
        .map_err(napi::Error::from_reason)?;

    let out = retrieve_memory(
        q,
        engine_candidates.into_iter(),
        engine::RetrieveOptions {
            top_k: opts.top_k as usize,
            min_score: opts.min_score as f32,
        },
    );

    Ok(out.into_iter().map(Into::into).collect())
}
