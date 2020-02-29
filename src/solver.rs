use std::fs::File;
use std::io::Read;

use crate::{
	genetic::solve as genetic_solve,
	ast::Simulation,
	genetic_config_parser::parse_genetic_config,
	utils::generalize_error,
};

pub enum Algorithm {
	Genetic
}

pub type Duration = usize;
pub type Steps = Vec<String>;
type Batch = (Duration, Steps);
pub type Production = Vec<Batch>;

// TODO: unmock
fn get_algorithm(_simulation: &Simulation) -> Algorithm {
	Algorithm::Genetic
}

pub fn solve(simulation: Simulation) -> Result<Production, String> {
	let algorithm = get_algorithm(&simulation);

	match algorithm {
		Algorithm::Genetic => {
			let mut genetic_config_file = File::open("generic_config.json").map_err(generalize_error)?;
			let mut genetic_config_content = String::new();
			genetic_config_file.read_to_string(&mut genetic_config_content).unwrap();
			let genetic_config = parse_genetic_config(genetic_config_content)?;
			genetic_solve(simulation, genetic_config).map_err(generalize_error)
		}
	}
}
