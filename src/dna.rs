use crate::score::Score;
use crate::solver::{Path, Step};

pub struct DNA {
  path: Path,
  favorite_steps: Vec<Step>,
  probability: f32,
}

pub fn build_dnas(parents: Vec<(Score, Path)>) -> Vec<DNA> {
  unimplemented!()
}
