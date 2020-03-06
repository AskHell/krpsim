use std::fs::File;
use std::io::Read;
use std::cmp::max;

use crate::{
	genetic::solve as genetic_solve,
	ast::{Simulation, Process},
	genetic_config_parser::parse_genetic_config,
	utils::generalize_error,
	check::{consume_resources, produce_resources}
};

pub enum Algorithm {
	Genetic
}

pub type Duration = usize;
pub type Step = String;
pub type Path = Vec<Step>;
type Batch = (Duration, Path);
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


pub fn batchify(simulation: &Simulation, process_names: Path) -> Result<Production, String> {
	let processes: Vec<&Process> = process_names.iter().map(|process_name| {
		simulation.processes.get(process_name).ok_or(format!("No process found: {}", process_name)).clone()
	})
	.collect::<Result<Vec<&Process>, String>>()?;

	let mut batched_processes = vec![];
	let mut current_batch = (0, vec![]);
	let start_stock = simulation.inventory.clone();
	let mut batch_stock = simulation.inventory.clone();
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
				let processes: Vec<&Process> = current_batch.1
					.iter()
					.map(|batch_process_name| {
						simulation.processes.get(batch_process_name).ok_or(format!("No process found: {}", batch_process_name)).clone()
					})
					.collect::<Result<Vec<&Process>, String>>()?;

				batch_stock = processes
					.iter()
					.try_fold(start_stock.clone(), |acc, process| { 
						produce_resources(&process.output, acc)
					})?;
				current_batch = (0, vec![]);
			}
		}
	}
	batched_processes.push(current_batch);
	Ok(batched_processes)
}
