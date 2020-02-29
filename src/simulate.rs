use crate::solver::{Path, Duration, batchify};
use crate::inventory::{Inventory};
use crate::ast::{Simulation};
use crate::check::{manage_resources};

fn simulate_steps(simulation: &Simulation, steps: &Path, stock: Inventory) -> Inventory {
	steps.iter().map(|process_name| {
		simulation.processes.get(process_name).unwrap().clone() // TODO: protect
	})
	.fold(stock.clone(), |acc, process| {
		manage_resources(acc, &process).unwrap() // TODO: protect
	})
}

fn simulate_path(simulation: &Simulation, path: &Path) -> (Inventory, Duration) {
	let simulation_inventory = simulation.inventory.clone();
	(simulate_steps(simulation, path, simulation_inventory), 0)
}

fn simulate_batch(simulation: &Simulation, path: &Path) -> (Inventory, Duration) {
	let simulation_inventory = simulation.inventory.clone();
	let initial_acc = (simulation_inventory, 0);
	let production = batchify(simulation, path.clone());
	production
		.iter()
		.fold(initial_acc, |(stock, duration), (step_duration, step_processes)| {
			let new_stock = simulate_steps(simulation, &step_processes, stock);
			(new_stock.clone(), duration + step_duration)
		})
}

pub fn simulate(simulation: &Simulation, path: &Path, with_time: bool) -> (Inventory, Duration) {
	if with_time {
		simulate_batch(simulation, path)
	} else {
		simulate_path(simulation, path)
	}
}
