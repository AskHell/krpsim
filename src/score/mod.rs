mod leo;
mod hugo;

use std::collections::HashMap;

use crate::ast::Simulation;
use crate::solver::{Path};
use leo::{build_score_map_leo, leo_score};
use hugo::{build_score_map_hugo, hugo_score};

pub type Score = i32;
pub type Weight = usize;
pub type ScoreMap = HashMap<String, Score>;

#[derive(Clone, Copy)] // Remove 'Copy' if you change the enum
pub enum BroScore {
	Leo,
    Hugo,
}

fn build_score_map(simulation: &Simulation, weight_multiplier: usize, broScore: BroScore) -> ScoreMap {
	match broScore {
		BroScore::Leo => build_score_map_leo(simulation, weight_multiplier),
        BroScore::Hugo => build_score_map_hugo(simulation, weight_multiplier),
	}
}

pub struct Scorer {
	simulation: Simulation,
	score_map: ScoreMap,
	time_weight: f32,
    bro_score: BroScore,
}

impl Scorer {
	pub fn new(simulation: Simulation, time_weight: f32, bro_score: BroScore) -> Self {
		let score_map = build_score_map(&simulation, 100, bro_score);
		Self {
			simulation: simulation.clone(),
			score_map,
			time_weight: if simulation.optimize_time { time_weight } else { 0. },
            bro_score,
		}
	}

	// TODO: memoize
	pub fn score(&self, path: &Path) -> Result<Score, String> {
        match self.bro_score {
            BroScore::Leo => leo_score(&self.simulation, &self.score_map, self.time_weight, path),
            BroScore::Hugo => hugo_score(),
        }
	}
}
