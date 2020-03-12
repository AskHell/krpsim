use std::collections::HashMap;

use crate::ast::{self, Simulation};
use super::{
    Score,
    ScoreMap,
};

pub fn build_score_map_hugo(simulation: &Simulation, _weight_multiplier: usize) -> ScoreMap {
    unimplemented!();
    // let score_map: ScoreMap = simulation.processes.iter().map(|p| (p, 0)).
}

pub fn hugo_score() -> Result<Score, String> {
    unimplemented!();
}
