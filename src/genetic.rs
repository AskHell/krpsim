extern crate serde;
extern crate rand;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cmp::max;

use crate::{
	ast::{Simulation, Process},
	inventory::Inventory,
	check::{manage_resources, consume_resources, produce_resources},
	genetic_plot::plot,
	solver::Production,
};

type Duration = usize;
type Batch = (Duration, Vec<String>);

#[derive(Serialize, Deserialize)]
pub struct Config {
	mutation_chance: f32,
	max_depth: usize,
	generation_size: usize,
	iterations: usize,
}

#[derive(Clone)]
pub struct Stats {
	pub average_scores: Vec<usize>,
	pub best_scores: Vec<usize>,
}

impl Stats {
	pub fn new() -> Self {
		Self {
			average_scores: vec![],
			best_scores: vec![],
		}
	}

	// Arguments: sorted scores
	pub fn update_scores(&mut self, generation_scores: Vec<usize>) {
		let average_score = generation_scores.iter().sum::<usize>() / generation_scores.len();
		let best_generation_score = generation_scores.get(0).unwrap_or(&0);
		self.average_scores.push(average_score);
		self.best_scores.push(*best_generation_score);
	}
}

struct GeneticSolver {
	mutation_chance: f32,
	max_depth: usize,
	generation_size: usize,
	iterations: usize,
	weigths: Vec<usize>,
	simulation: Simulation,
	stats: Stats
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

pub fn solve<'a>(simulation: Simulation, config: Config) -> Result<Production, &'a str> {
	let mut solver = GeneticSolver::new(config, simulation.clone());
	solver.solve()
	.map(|(production, stats)| {
		plot(stats);
		production
	})
}

impl GeneticSolver {
	pub fn new(config: Config, simulation: Simulation) -> Self {
		let mut solver = Self {
			mutation_chance: config.mutation_chance,
			max_depth: config.max_depth,
			generation_size: config.generation_size,
			iterations: config.iterations,
			simulation,
			weigths: fibonacci_n(config.generation_size),
			stats: Stats::new(),
		};
		solver.weigths.reverse();
		solver
	}

	pub fn solve<'a>(&mut self) -> Result<(Production, Stats), &'a str> {
		let mut parents = vec![];
		for i in 0..self.iterations {
			let generation = if i == 0 {
				self.generate()
			} else {
				self.shuffle(parents)
			};
			parents = self.select(generation);
		}
		let best = parents.into_iter()
			.max_by(|pa, pb| {
				self.score(pa.clone())
				.cmp(
					&self.score(pb.clone())
				)
			})
			.unwrap_or(vec![]);
		Ok((best, self.stats.clone()))
	}

	fn shuffle(&self, productions: Vec<Production>) -> Vec<Production> {
		let mut rng = rand::thread_rng();
		productions.into_iter()
		.zip(self.weigths.iter())
		.fold(vec![], |mut acc, (production, weight)| {
			let mut to_append: Vec<Production> = (0..*weight).map(|_| {
				let i = rng.gen_range(0.0, 1.);
				let new_production = if i <= self.mutation_chance {
					self.generate_one()
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

	fn get_available_steps(&self, inventory: &Inventory) -> Vec<(String, Inventory)> {
		self.simulation.processes.clone().into_iter().map(|(name, process)| {
			manage_resources(inventory.clone(), &process)
			.map(|inventory| { (name, inventory) })
		})
		.filter(|x| { x.is_ok() })
		.map(|x| { x.unwrap_or(("shouldn't be here".to_string(), HashMap::new())) })
		.map(|(name, inv)| { (name.clone(), inv) })
		.collect()
	}

	fn generate_one(&self) -> Production {
		let mut production: Production = vec![];
		let mut rng = rand::thread_rng();
		let mut simulation_inventory = self.simulation.inventory.clone();
		for _ in 0..self.max_depth {
			let available_steps = self.get_available_steps(&simulation_inventory);
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
	fn generate(&self) -> Vec<Production> {
		(0..self.generation_size).map(|_| {
			self.generate_one()
		})
		.collect()
	}

	fn simulate(&self, production: &Production) -> (Inventory, usize) {
		let simulation_inventory = self.simulation.inventory.clone();
		let initial_acc = (simulation_inventory, 0);
		production
			.iter()
			.fold(initial_acc, |(inventory, n_steps), process_name| {
				let process = self.simulation.processes.get(process_name).unwrap();
				let new_inventory = manage_resources(inventory, &process).unwrap();
				// println!("{:?}", new_inventory);
				(new_inventory, n_steps + 1)
			})
	}

	// TODO: take time into account
	fn score(&self, production: Production) -> (usize, Production) {
		let (inventory, _n_steps) = self.simulate(&production);
		let stock_score = self.simulation.optimize.iter().fold(0, |acc, key| {
			let resource_score = inventory.get(key).unwrap_or(&0);
			acc + resource_score
		});
		// if self.simulation.optimize_time {
		// 	return if stock_score >= n_steps {
		// 		(stock_score - n_steps, updated_production)
		// 	} else {
		// 		(0, updated_production)
		// 	}
		// }
		(stock_score, production)
	}

	// return best 10% of the population sorted
	fn select(&mut self, productions: Vec<Production>) -> Vec<Production> {
		let mut p_scores: Vec<(usize, Production)> = productions.iter().map(|production| {
			self.score(production.clone())
		})
		.collect();
		// DEBUG
		// - average score
		// - average size
		p_scores.sort_by(|(score_a, _a), (score_b, _b)| { score_b.cmp(score_a) });
		self.stats.update_scores(p_scores.iter().map(|p_score| { p_score.0 }).collect());
		let best: Vec<Production> = p_scores.iter().take(self.generation_size / 10).map(|p_score| { p_score.clone().1 }).collect();
		best
	}

	fn batchify(&self, process_names: Vec<String>) -> Vec<Batch> {
		let processes: Vec<Process> = process_names.into_iter().map(|process_name| {
			self.simulation.processes.get(&process_name).unwrap().clone() // TODO: protect!
		}).collect();
		let mut batched_processes = vec![];
		let mut current_batch = (0, vec![]);
		let start_stock = self.simulation.inventory.clone();
		let mut batch_stock = self.simulation.inventory.clone();
		for process in processes {
			match consume_resources(&process.input, batch_stock.clone()).ok() {
				Some (updated_stock) => {
					batch_stock = updated_stock;
					let (duration, batch_processes) = current_batch.clone();
					let new_duration = max(duration, process.duration);
					let new_batch_processes = [&batch_processes[..], &[process.name.clone()]].concat();
					current_batch = (new_duration, new_batch_processes);
				}
				None => {
					batched_processes.push(current_batch.clone());
					batch_stock = current_batch.1
						.iter()
						.map(|batch_process_name| { self.simulation.processes.get(batch_process_name).unwrap().clone() }) // TODO: protect
						.fold(start_stock.clone(), |acc, process| { 
							produce_resources(&process.output, acc).unwrap() // TODO: protect
						})
				}
			}
		}
		batched_processes
	}
}
