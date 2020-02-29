use crate::ast::{Simulation};
use crate::solver::{Path};
use crate::simulate::simulate;

pub type Score = i32;

pub fn score(simulation: &Simulation, path: Path, time_weight: f32) -> (Score, Path) {
	let (inventory, duration) = simulate(simulation, &path, simulation.optimize_time);
	let stock_score = simulation.optimize.iter().fold(0, |acc, key| {
		let resource_score = inventory.get(key).unwrap_or(&0);
		acc + *resource_score as Score
	});
	let time_score = (duration as f32 * time_weight) as Score;
	let score: Score = stock_score - ((time_score as f32 * time_weight) as Score); 
	(score, path)
}
