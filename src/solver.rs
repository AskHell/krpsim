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
	weigths: Vec<usize>,
	simulation: Simulation
}

fn fibonacci_n(n: usize) -> Vec<usize> {
    let (list, _) = (1..100).fold((vec![], 0), |(mut acc, total), i| {
        if total > n {
            return (acc, total)
        }
        let to_append = if i < 3 {
			1
		} else {
		    let a = acc.get(i - 3).unwrap_or(&0);
			let b = acc.get(i - 2).unwrap_or(&1);
			a + b
		};
		let new_total = total + to_append;
		if new_total > n {
			let mut padding = vec![1; n - total];
			padding.append(&mut acc);
			return (padding, new_total)
		}
        acc.push(to_append);
        (acc, new_total)
    });
	list
}

impl Solver {
	pub fn new(mutation_chance: f32, max_depth: usize, generation_size: usize, iterations: usize, simulation: Simulation) -> Self {
		let mut solver = Self {
			mutation_chance,
			max_depth,
			generation_size,
			iterations,
			simulation,
			weigths: fibonacci_n(generation_size)
		};
		solver.weigths.reverse();
		solver
	}

	pub fn solve(&self, simulation: &Simulation) -> Result<Production, String> {
		let mut parents = vec![];
		for i in 0..self.iterations {
			let generation = if i == 0 {
				self.generate(&simulation)
			} else {
				self.shuffle(parents)
			};
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

	fn shuffle(&self, productions: Vec<Production>) -> Vec<Production> {
		let mut rng = rand::thread_rng();
		productions.into_iter()
		.zip(self.weigths.iter())
		.fold(vec![], |mut acc, (production, weight)| {
			let mut to_append: Vec<Production> = (0..*weight).map(|_| {
				let i = rng.gen_range(0.0, 1.);
				let new_production = if i <= self.mutation_chance {
					println!("MUTATION!!");
					self.generate_one(&self.simulation)
				} else {
					production.clone()
				};
				new_production
			})
			.collect();
			acc.append(&mut to_append);
			acc
		})
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

	fn generate_one(&self, simulation: &Simulation) -> Production {
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
		};
		production
	}
	// First random generation, doable paths
	fn generate(&self, simulation: &Simulation) -> Vec<Production> {
		(0..self.generation_size).map(|_| {
			self.generate_one(simulation)
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

	// return best 10% of the population sorted
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
		best
	}
}
