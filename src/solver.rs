use std::fs::File;
use std::io::Read;

use crate::{
	genetic::solve as genetic_solve,
	ast::{Simulation, Process},
	genetic_config_parser::parse_genetic_config,
	utils::generalize_error,
	check::{consume_resources, manage_multi_resources}
};

pub enum Algorithm {
	Genetic
}

pub type Duration = usize;
pub type Step = String;
pub type Path = Vec<Step>;
pub type Batch = (Duration, Path);
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

fn create_batch(processes: Vec<&Process>) -> Result<Batch, String> {
	let duration = processes
		.iter()
		.map(|process| { process.duration} )
		.max()
		.ok_or("Can't find the longest process, probably an empty iterator")?;
	let process_names = processes.iter().map(|process| { process.name.clone() } ).collect();
	Ok((duration, process_names))
}

// TODO: memoize
pub fn batchify(simulation: &Simulation, process_names: Path) -> Result<Production, String> {
	let processes: Vec<&Process> = process_names
		.into_iter().map(|process_name| {
		simulation.processes.get(&process_name).ok_or(format!{"Unable to find process: {}", process_name})
	})
	.collect::<Result<Vec<&Process>, String>>()?;

	let mut batch_processes: Vec<&Process> = vec![];
	let mut inventory = simulation.inventory.clone();
	let mut base_inventory = simulation.inventory.clone();
	let mut batched: Vec<Batch> = vec![];
	let batched_res: Result<Vec<Batch>, String> = processes.iter().try_fold(batched, |mut batched, process| {
		match consume_resources(&process.input, inventory.clone()).ok() {
			Some (updated_inventory) => {
				batch_processes.push(&process);
				inventory = updated_inventory;
				Ok(batched)
			},
			None => {
				let tmp = inventory.clone();
				inventory = manage_multi_resources(base_inventory.clone(), batch_processes.clone())?;
				base_inventory = tmp;
				let batch = create_batch(batch_processes.clone())?;
				batched.push(batch);
				batch_processes = vec![&process];
				Ok(batched)
			}
		}
	});
	batched = batched_res?;
	if !batch_processes.is_empty() {
		let batch = create_batch(batch_processes)?;
		batched.push(batch);
	};
	Ok(batched)
}
