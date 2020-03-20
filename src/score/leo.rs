use std::collections::HashMap;

use super::{Score, ScoreMap};
use crate::ast::Simulation;
use crate::simulate::simulate;
use crate::solver::Path;

pub fn build_score_map_leo(simulation: &Simulation, _weight_multiplier: usize) -> ScoreMap {
	let score_weigth = 1;
	simulation
		.optimize
		.iter()
		.fold(HashMap::new(), |mut acc, resource_name| {
			acc.insert(resource_name.clone(), score_weigth);
			acc
		})
}

pub fn leo_score(
	simulation: &Simulation,
	score_map: &ScoreMap,
	time_weight: f32,
	path: &Path,
) -> Result<Score, String> {
	let (inventory, duration) = simulate(&simulation, &path, false)?;
	let stock_score = inventory.into_iter().fold(0, |score, (name, _)| {
		score + *score_map.get(&name).unwrap_or(&0)
	});
	let time_score = duration as f32 * time_weight;
	let score = stock_score as Score - time_score.round() as Score;
	Ok(score)
}
