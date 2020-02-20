extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use super::ast::{Simulation, Process};
use super::inventory::Inventory;
use super::check::manage_resources;

type Production = Vec<String>;

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

	fn shuffle(&self, productions: Vec<Production>) -> Vec<Production> {
		for i in 0..productions[0].len() {
			let steps: Vec<String> = productions.iter().map(|production| { production[i] }).collect();
			let mut slice: &mut [String] = &mut steps;
    		let mut rng = thread_rng();

    		let mut y = [1, 2, 3, 4, 5];
			let mut new_steps: Vec<Production> = vec![];
			for i in 0..10 {
	    		slice.shuffle(&mut rng);
			}
		}
		vec![]
	}

	// TODO: implement
	fn generate(&self, parents: Vec<Production>) -> Vec<Production> {
		vec![]
	}

	// production can be returned trimed
	fn simulate(&self, simulation: &Simulation, production: Production) -> (Inventory, usize, Production) {
		let mut simulation_inventory = simulation.inventory.clone();
		let mut updated_production = vec![];
		let mut i = 0;
		for step in production {
			match simulation.processes.get(&step) {
				Some (process) => {
					match manage_resources(simulation_inventory, process) {
						Ok (updated_inventory) => {
							simulation_inventory = updated_inventory;
							updated_production.push(step);
							i += 1;
						}
						Err (_err) => {
							return (simulation_inventory, i, updated_production)
						}
					}
				}
				None => {
					return (simulation_inventory, i, updated_production)
				}
			}
		}
		(simulation_inventory, i, updated_production)
	}

	fn score(&self, production: Production, simulation: &Simulation) -> (usize, Production) {
		let (inventory, n_steps, updated_production) = self.simulate(simulation, production);
		let stock_score = simulation.optimize.iter().fold(0, |acc, key| {
			let resource_score = inventory.get(key).unwrap_or(&0);
			acc + resource_score
		});
		if simulation.optimize_time {
			return if stock_score >= n_steps {
				(stock_score - n_steps, updated_production)
			} else {
				(0, updated_production)
			}
		}
		(stock_score, updated_production)
	}

	// Trim selected ones to the shortest
	fn select(&self, productions: Vec<Production>, simulation: &Simulation) -> Vec<Production> {
		let p_scores: Vec<(usize, Production)> = productions.iter().map(|production| {
			self.score(*production, simulation)
		})
		.collect();
		// DEBUG
		// - average score
		// - average size
		p_scores.sort_by(|(score_a, _a), (score_b, _b)| { score_b.cmp(score_a) });
		let best: Vec<Production> = p_scores.iter().take(self.generation_size / 10).map(|p_score| { p_score.1 }).collect();
		let shortest = best.iter().map(|a| { a.len() }).min().unwrap_or(0);
		// DEBUG: shortest
		best.into_iter().map(|a| {
			if a.len() > shortest {
				return a.into_iter().take(shortest).collect()
			}
			a
		}).collect()
	}
}
