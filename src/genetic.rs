extern crate rand;
extern crate serde;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
	ast::Simulation,
	check::manage_resources,
	dna::{build_dnas, DNA},
	genetic_plot::plot,
	inventory::Inventory,
	score::{BroScore, Score, Scorer},
	solver::{batchify, Path, Production, Step},
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
	parents_size: usize,
	iterations: usize,
	weigths: Vec<usize>,
	simulation: Simulation,
	stats: Stats,
	scorer: Scorer,
}

pub fn solve<'a>(simulation: Simulation, config: Config) -> Result<Production, String> {
	let mut solver = GeneticSolver::new(config, simulation.clone());
	solver.solve().map(|(production, stats)| {
		plot(stats);
		production
	})
}

impl GeneticSolver {
	// TODO: parents_percentage in config
	pub fn new(config: Config, simulation: Simulation) -> Self {
		let parents_percentage = 10;
		let parents_size = if config.generation_size / parents_percentage > 1 {
			config.generation_size / parents_percentage
		} else {
			1
		};
		let mut solver = Self {
			mutation_chance: config.mutation_chance,
			max_depth: config.max_depth,
			generation_size: config.generation_size,
			parents_size,
			iterations: config.iterations,
			simulation: simulation.clone(),
			weigths: fibonacci_n(config.generation_size),
			stats: Stats::new(),
			scorer: Scorer::new(simulation, config.time_weight, BroScore::Leo),
		};
		solver.weigths.reverse();
		solver
	}

	pub fn solve(&mut self) -> Result<(Production, Stats), String> {
		let mut parents: Vec<(Score, Path)> = vec![];
		for i in 0..self.iterations {
			let generation = if i == 0 {
				self.generate()
			} else {
				self.reproduce(parents)?
			};
			parents = self.select(generation);
		}
		// Find creme de la creme
		unimplemented!()
	}

	// Should only return possible steps
	fn build_step_choices(
		&self,
		available_steps: Vec<&Step>,
		dnas: &Vec<DNA>,
		index: usize,
	) -> Vec<Step> {
		dnas.iter().fold(vec![], |acc, current| {
			let steps = current.pick_steps(index, &available_steps);
			[&acc[..], &steps[..]].concat()
		})
	}

	fn select_step(&self, available_steps: Vec<&Step>, dnas: &Vec<DNA>, index: usize) -> Step {
		let choices = self.build_step_choices(available_steps, dnas, index);
		pick_random(choices)
	}

	fn reproduce(&self, parents: Vec<(Score, Path)>) -> Result<Vec<Path>, String> {
		let mut inventory = &self.simulation.inventory.clone();
		let dnas = build_dnas(parents);
		let mut paths: Vec<Path> = vec![];
		let mut available_steps_i: HashMap<String, Inventory>;
		for _ in 0..self.generation_size {
			let mut path: Path = vec![];
			for i in 0..self.max_depth {
				available_steps_i = self.get_available_steps(inventory);
				if available_steps_i.is_empty() {
					return Ok(paths);
				}
				let available_steps: Vec<&Step> = available_steps_i.keys().collect();
				let step = self.select_step(available_steps, &dnas, i);
				inventory = available_steps_i
					.get(&step)
					.ok_or(format!("Unable to find step {}", step))?;
				path.push(step);
			}
			paths.push(path);
		}
		Ok(paths)
	}

	fn get_available_steps(&self, inventory: &Inventory) -> HashMap<Step, Inventory> {
		self
			.simulation
			.processes
			.clone()
			.into_iter()
			.map(|(name, process)| {
				manage_resources(inventory.clone(), &process).map(|inventory| (name, inventory))
			})
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap_or(("shouldn't be here".to_string(), HashMap::new())))
			.map(|(name, inv)| (name.clone(), inv))
			.collect()
	}

	fn generate_one(&self, len: usize, base_inventory: &Inventory) -> Path {
		let mut production: Path = vec![];
		let mut simulation_inventory = base_inventory.clone();
		for _ in 0..len {
			let available_steps = self.get_available_steps(&simulation_inventory);
			if available_steps.is_empty() {
				return production;
			}
			let keys: Vec<&Step> = available_steps.keys().collect();
			let step_name = pick_random(keys);
			simulation_inventory = available_steps.get(step_name).unwrap().clone(); //TODO: protect
			production.push(step_name.clone());
		}
		production
	}
	// First random generation, doable paths
	fn generate(&self) -> Vec<Path> {
		(0..self.generation_size)
			.map(|_| self.generate_one(self.max_depth, &self.simulation.inventory))
			.collect()
	}

	// return top 10% of the population, sorted
	fn select(&mut self, paths: Vec<Path>) -> Vec<(Score, Path)> {
		let mut p_scores: Vec<(Score, Path)> = paths
			.into_iter()
			.map(|path| {
				let score = self.scorer.score(&path).unwrap_or(-1);
				(score, path)
			})
			.collect();
		p_scores.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(score_a));
		self
			.stats
			.update_scores(p_scores.iter().map(|p_score| p_score.0).collect());
		let bests: Vec<(Score, Path)> = p_scores.into_iter().take(self.parents_size).collect();
		bests
	}
}

fn pick_random<T: Clone>(vec: Vec<T>) -> T {
	let mut rng = rand::thread_rng();
	let i = rng.gen_range(0, vec.len());
	return vec[i].clone();
}
