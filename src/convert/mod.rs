use crate::structs::{JudgeSpecs, Problem};

pub mod io_fast;

pub trait Converter {
    fn convert(&self, input: &Problem) -> JudgeSpecs;
}
