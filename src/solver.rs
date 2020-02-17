use super::ast::{Simulation, Process};

type Production = Vec<Process>;

struct Solver {
	mutation_chance: f32,
	max_depth: usize,
	generation_size: usize,
}

impl Solver {
	pub fn new(mutation_chance: f32, max_depth: usize, generation_size: usize) -> Self {
		Self {
			mutation_chance,
			max_depth,
			generation_size,
		}
	}

	// TODO: implement
	pub fn solve(simulation: &Simulation) -> Result<Production, String> {
		Ok(vec![])
	}

	// TODO: implement
	fn generate(parents: Vec<Production>) -> Vec<Production> {
		vec![]
	}

	// TODO: implement
	fn score(production: Production, optimize: Vec<String>, optimize_time: bool) -> usize {
		0
	}

	// TODO: implement
	fn select(productions: Vec<Production>, optimize: Vec<String>, optimize_time: bool) -> Vec<Production> {
		vec![]
	}
}
