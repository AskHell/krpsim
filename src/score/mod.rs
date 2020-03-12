mod leo;
mod hugo;

use std::collections::HashMap;

use crate::ast::Simulation;
use crate::solver::{Path};
use crate::simulate::simulate;
use leo::build_score_map_leo;
use hugo::build_score_map_hugo;

pub type Score = i32;
pub type Weight = usize;
pub type ScoreMap = HashMap<String, Score>;

pub enum BroScore {
	Leo,
    Hugo
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
}

impl Scorer {
	pub fn new(simulation: Simulation, time_weight: f32, bro_score: BroScore) -> Self {
		let score_map = build_score_map(&simulation, 100, bro_score);
		Self {
			simulation: simulation.clone(),
			score_map,
			time_weight: if simulation.optimize_time { time_weight } else { 0. },
		}
	}

	// TODO: memoize
	pub fn score(&self, path: &Path) -> Result<Score, String> {
		let (inventory, duration) = simulate(&self.simulation, &path, false)?;
		let stock_score =
			inventory
			.into_iter()
			.fold(0, |score, (name, _)| {
				score + *self.score_map.get(&name).unwrap_or(&0)
			});
		let time_score = duration as f32 * self.time_weight;
		let score = stock_score as Score - time_score.round() as Score;
		Ok(score)
	}
}
