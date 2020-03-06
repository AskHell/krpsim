use crate::solver::{Path, Duration, batchify};
use crate::inventory::{Inventory};
use crate::ast::{Simulation, Process};
use crate::check::{manage_resources};

fn simulate_steps(simulation: &Simulation, steps: &Path, stock: Inventory) -> Result<Inventory, String> {
	let processes: Vec<&Process> = steps
		.iter().map(|process_name| {
			simulation.processes.get(process_name).ok_or(format!("Unknown process: {:?}", process_name))
		})
		.collect::<Result<Vec<&Process>, String>>()?;

	let inventory: Inventory = processes
		.iter()
		.try_fold(stock.clone(), |acc, process| {
			manage_resources(acc, &process)
		})?;

	Ok(inventory)
}

fn simulate_path(simulation: &Simulation, path: &Path) -> Result<(Inventory, Duration), String> {
	let simulation_inventory = simulation.inventory.clone();
	let simulation_inventory = simulate_steps(simulation, path, simulation_inventory)?;
	Ok((simulation_inventory, 0))
}

fn simulate_batch(simulation: &Simulation, path: &Path) -> Result<(Inventory, Duration), String> {
	let simulation_inventory = simulation.inventory.clone();
	let initial_acc = (simulation_inventory, 0);
	let production = batchify(simulation, path.clone())?;
	production
		.iter()
		.try_fold(initial_acc, |(stock, duration), (step_duration, step_processes)| {
			let new_stock = simulate_steps(simulation, &step_processes, stock)?;
			Ok((new_stock.clone(), duration + step_duration))
		})
}

pub fn simulate(simulation: &Simulation, path: &Path, with_time: bool) -> Result<(Inventory, Duration), String> {
	if with_time {
		simulate_batch(simulation, path)
	} else {
		simulate_path(simulation, path)
	}
}
