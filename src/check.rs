use itertools::Itertools;

use super::inventory::Inventory;
use super::ast::Process;
use super::ast::Simulation;

pub struct Output {
	pub steps: Vec<String>
}

pub fn manage_resources<'a>(inventory: Inventory, process: &Process) -> Result <Inventory, &'a str> {
	let original_acc = inventory.clone();
	process.input
		.iter()
		.map(Ok::<_, ()>)
		.fold_results(original_acc, |mut acc, resource| {
			let n_items = acc.get(&resource.name)?;
			acc.insert(resource.name.clone(), n_items - resource.quantity);
			Ok(acc)
		});
	Ok (inventory)
	// for resource_needed in &process.input {
	// 	match inventory.clone().get(&resource_needed.name) {
	// 		Some (n_items) => {
	// 			if n_items < &resource_needed.quantity {
	// 				return Err ("Not enough available resources.")
	// 			}
	// 			inventory.insert(resource_needed.name.clone(), *n_items - resource_needed.quantity);
	// 		}
	// 		None => {
	// 			return Err ("Unexisting resource.")
	// 		}
	// 	}
	// }
	// for resource_created in &process.output {
	// 	match inventory.clone().get(&resource_created.name) {
	// 		Some (n_items) => {
	// 			inventory.insert(resource_created.name.clone(), *n_items + resource_created.quantity);
	// 		}
	// 		None => {
	// 			inventory.insert(resource_created.name.clone(), resource_created.quantity);
	// 		}
	// 	}
	// }
	// Ok (inventory)
}

pub fn check<'a>(simulation: Simulation, output: Output) -> Result <Inventory, &'a str> {
	output.steps.iter()
	.fold(Ok(simulation.inventory.clone()), |inventory_res, step| {
		match inventory_res {
			Err (err) => {
				Err (err)
			}
			Ok (inventory) => {
				match simulation.processes.get(step) {
					Some (process) => {
						manage_resources(inventory, process)
					},
					None => {
						Err ("Error, to lazy to find proper name")
					}
				}
			}
		}
	})
}
