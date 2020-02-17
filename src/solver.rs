use std::collections::HashMap;

use super::ast::{Simulation, Process};
use super::inventory::Inventory;

type Production = Vec<Process>;

struct Solver {
	mutation_chance: f32,
	max_depth: usize,
	generation_size: usize,
	iterations: usize,
}


impl Solver {
	pub fn new(mutation_chance: f32, max_depth: usize, generation_size: usize, iterations: usize) -> Self {
		Self {
			mutation_chance,
			max_depth,
			generation_size,
			iterations,
		}
	}

	pub fn solve(&self, simulation: &Simulation) -> Result<Production, String> {
		let mut parents = vec![];
		for i in 0..self.iterations {
			let generation = self.generate(parents);
			parents = self.select(generation, &simulation);
		}
		let best = parents.iter()
			.max_by(|pa, pb| {
				self.score(pa, &simulation)
				.cmp(
					&self.score(pb, &simulation)
				)
			})
			.unwrap_or(&vec![])
			.clone();
		Ok(best)
	}

	// TODO: implement
	fn generate(&self, parents: Vec<Production>) -> Vec<Production> {
		vec![]
	}

	// TODO: implement
	fn simulate(&self, simulation: &Simulation, production: &Production) -> (Inventory, usize) {
		(HashMap::new(), 0)
	}

	fn score(&self, production: &Production, simulation: &Simulation) -> usize {
		let (inventory, n_steps) = self.simulate(simulation, production);
		let stock_score = simulation.optimize.iter().fold(0, |acc, key| {
			let resource_score = inventory.get(key).unwrap_or(&0);
			acc + resource_score
		});
		if simulation.optimize_time {
			return if stock_score >= n_steps {
				stock_score - n_steps
			} else {
				0
			}
		}
		stock_score
	}

	// TODO: implement
	fn select(&self, productions: Vec<Production>, simulation: &Simulation) -> Vec<Production> {
		// let scores: Vec<usize> = generation.iter().map(|production| {
		// 	self.score(production, &simulation.optimize, simulation.optimize_time)
		// }).collect();
		vec![]
	}
}
