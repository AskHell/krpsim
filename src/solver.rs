extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::collections::HashMap;

use super::ast::{Simulation};
use super::inventory::Inventory;
use super::check::manage_resources;

type Production = Vec<String>;

pub struct Solver {
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
		for _ in 0..self.iterations {
			let generation = self.generate(parents, &simulation);
			parents = self.select(generation, &simulation);
		}
		let best = parents.into_iter()
			.max_by(|pa, pb| {
				self.score(pa.clone(), &simulation)
				.cmp(
					&self.score(pb.clone(), &simulation)
				)
			})
			.unwrap_or(vec![]);
		Ok(best)
	}

	// productions should all have equal lens here
	fn shuffle(&self, productions: Vec<Production>) -> Vec<Production> {
    	let mut rng = thread_rng();
		let mut shuffled_production: Vec<Production> = vec![];
		// TODO: find a better way
		for _ in 0..100 {
			shuffled_production.push(vec![]);
		}
		for i in 0..productions[0].len() {
			let steps: Vec<String> = productions.iter().map(|production| { production[i].clone() }).collect();
			let mut shuffled_steps = steps.clone();
			let mut new_steps: Vec<String> = vec![];
			for _ in 0..9 {
				shuffled_steps.shuffle(&mut rng);
				new_steps.append(&mut shuffled_steps.clone());
			}
			for step in steps.into_iter() {
				// TODO: add mutation chance here
				new_steps.push(step);
			}
			for y in 0..new_steps.len() {
				shuffled_production[y].push(new_steps[y].clone());
			}
		}
		shuffled_production
	}

	fn get_available_steps(&self, inventory: &Inventory, simulation: &Simulation) -> Vec<(String, Inventory)> {
		simulation.processes.clone().into_iter().map(|(name, process)| {
			manage_resources(inventory.clone(), &process)
			.map(|inventory| { (name, inventory) })
		})
		.filter(|x| { x.is_ok() })
		.map(|x| { x.unwrap_or(("shouldn't be here".to_string(), HashMap::new())) })
		.map(|(name, inv)| { (name.clone(), inv) })
		.collect()
	}

	// First random generation, doable paths
	fn generate(&self, _parents: Vec<Production>, simulation: &Simulation) -> Vec<Production> {
		(0..self.generation_size).map(|_| {
			let mut production: Production = vec![];
			let mut rng = rand::thread_rng();
			let mut simulation_inventory = simulation.inventory.clone();
			for _ in 0..self.max_depth {
				let available_steps = self.get_available_steps(&simulation_inventory, &simulation);
				if available_steps.is_empty() {
					return production
				}
				let i = rng.gen_range(0, available_steps.len());
				let (step_name, updated_inventory) = available_steps[i].clone();
				production.push(step_name);
				simulation_inventory = updated_inventory;
			}
			production
		})
		.collect()
	}

	// production can be returned trimed
	fn simulate(&self, simulation: &Simulation, production: Production) -> (Inventory, usize, Production) {
		let mut simulation_inventory = simulation.inventory.clone();
		let mut updated_production = vec![];
		let mut i = 0;
		for step in production {
			match simulation.processes.get(&step) {
				Some (process) => {
					match manage_resources(simulation_inventory.clone(), process) {
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
		let mut p_scores: Vec<(usize, Production)> = productions.iter().map(|production| {
			self.score(production.clone(), simulation)
		})
		.collect();
		// DEBUG
		// - average score
		// - average size
		p_scores.sort_by(|(score_a, _a), (score_b, _b)| { score_b.cmp(score_a) });
		let best: Vec<Production> = p_scores.iter().take(self.generation_size / 10).map(|p_score| { p_score.clone().1 }).collect();
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
