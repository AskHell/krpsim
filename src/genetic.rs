extern crate serde;
extern crate rand;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
	ast::{Simulation},
	inventory::Inventory,
	check::{manage_resources},
	genetic_plot::plot,
	score::{Score, Scorer},
	solver::{Production, Path, batchify},
	simulate::{simulate},
	utils::fibonacci_n,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
	mutation_chance: f32,
	max_depth: usize,
	generation_size: usize,
	iterations: usize,
	time_weight: f32,
}

#[derive(Clone)]
pub struct Stats {
	pub average_scores: Vec<Score>,
	pub best_scores: Vec<Score>,
}

impl Stats {
	pub fn new() -> Self {
		Self {
			average_scores: vec![],
			best_scores: vec![],
		}
	}

	// Arguments: sorted scores
	pub fn update_scores(&mut self, generation_scores: Vec<Score>) {
		let average_score = generation_scores.iter().sum::<Score>() / generation_scores.len() as Score;
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
	stats: Stats,
	scorer: Scorer,
}

pub fn solve<'a>(simulation: Simulation, config: Config) -> Result<Production, String> {
	let mut solver = GeneticSolver::new(config, simulation.clone());
	solver.solve()
	.map(|(production, stats)| {
		// plot(stats);
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
			simulation: simulation.clone(),
			weigths: fibonacci_n(config.generation_size),
			stats: Stats::new(),
			scorer: Scorer::new(simulation, config.time_weight)
		};
		solver.weigths.reverse();
		solver
	}

	pub fn solve<'a>(&mut self) -> Result<(Production, Stats), String> {
		let mut parents: Vec<Path> = vec![];
		for i in 0..self.iterations {
			let generation = if i == 0 {
				self.generate()
			} else {
				self.shuffle(parents)?
			};
			parents = self.select(generation);
		}
		let best_path = parents.into_iter()
			.max_by(|pa, pb| {
				let score_a = self.scorer.score(pa);
				let score_b = self.scorer.score(pb);
				score_a.cmp(&score_b)
			})
			.unwrap_or(vec![]);
		let best_production = batchify(&self.simulation, best_path)?;
		Ok((best_production, self.stats.clone()))
	}

	fn mutate(&self, mutation_force: f32, mut path: Path) -> Result<Path, String> {
		let len = path.len();
		let split_at = len - (len as f32 * mutation_force) as usize - 1;
		path.truncate(split_at);
		let (base_inventory, _) = simulate(&self.simulation, &path, self.simulation.optimize_time)?;
		let rest = self.generate_one(len - split_at, &base_inventory);
		let mutated_path = [&path[..], &rest[..]].concat();
		Ok(mutated_path)
	}

	fn shuffle(&self, steps: Vec<Path>) -> Result<Vec<Path>, String> {
		let mut rng = rand::thread_rng();
		let mutation_mult = 1. / self.mutation_chance;
		steps
			.into_iter()
			.map(|path| {
				let i = rng.gen_range(0., 1.);
				if i <= self.mutation_chance {
					let mutation_force = i * mutation_mult;
					self.mutate(mutation_force, path)
				} else {
					Ok(path)
				}
			})
			.collect()
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

	fn generate_one(&self, len: usize, base_inventory: &Inventory) -> Path {
		let mut production: Path = vec![];
		let mut rng = rand::thread_rng();
		let mut simulation_inventory = base_inventory.clone();
		for _ in 0..len {
			let available_steps = self.get_available_steps(&simulation_inventory);
			if available_steps.is_empty() {
				return production
			}
			let i = rng.gen_range(0, available_steps.len());
			let (step_name, updated_inventory) = available_steps[i].clone();
			production.push(step_name.clone());
			simulation_inventory = updated_inventory;
		};
		production
	}
	
	// First random generation, doable paths
	fn generate(&self) -> Vec<Path> {
		(0..self.generation_size).map(|_| {
			self.generate_one(self.max_depth, &self.simulation.inventory)
		})
		.collect()
	}

	// return top 10% of the population, sorted
	fn select(&mut self, paths: Vec<Path>) -> Vec<Path> {
		let mut p_scores: Vec<(Score, Path)> = paths.into_iter().map(|path| {
			let score = self.scorer.score(&path).unwrap_or(-1);
			(score, path)
		})
		.collect();
		p_scores.sort_by(|(score_a, _), (score_b, _)| { score_b.cmp(score_a) });
		self.stats.update_scores(p_scores.iter().map(|p_score| { p_score.0 }).collect());
		let best: Vec<Path> = p_scores.iter().take(self.generation_size / 10).map(|p_score| { p_score.clone().1 }).collect();
		best
	}
}
