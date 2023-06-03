use std::path::Path;

use crate::structs::{JudgeSpecs, Problem};
use async_trait::async_trait;

pub mod io_fast;

#[async_trait]
pub trait Converter: Send + Sync {
    async fn convert(&self, problem: &Problem, dir: &Path) -> JudgeSpecs;
}
