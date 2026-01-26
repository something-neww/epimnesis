// NOTE:
// This is the stable Node FFI contract for Epimnesis.
// Breaking changes require a major version bump.

use engine::MemoryEngine;
use napi_derive::napi;

use crate::types::{
    JsRetrievalExplanation, JsRetrieveInput, JsRetrievedMemory,
};

mod convert;
mod types;

#[napi]
pub struct JsMemoryEngine {
    inner: MemoryEngine,
}

#[napi]
impl JsMemoryEngine {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: MemoryEngine::new(),
        }
    }

    #[napi]
    pub fn retrieve(&self, input: JsRetrieveInput) -> Vec<JsRetrievedMemory> {
        self.inner
            .retrieve(input.into())
            .into_iter()
            .map(Into::into)
            .collect()
    }

    #[napi]
    pub fn explain(&self, input: JsRetrieveInput) -> JsRetrievalExplanation {
        self.inner.explain(input.into()).into()
    }
}

impl Default for JsMemoryEngine {
    fn default() -> Self {
        Self::new()
    }
}
