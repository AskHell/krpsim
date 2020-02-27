use super::inventory::Inventory;
use super::ast::Process;
use super::ast::Simulation;

pub struct Output {
	pub steps: Vec<String>
}

// TODO: early exit (maybe using foldWhile?)
pub fn manage_resources<'a>(inventory: Inventory, process: &Process) -> Result <Inventory, &'a str> {
	let original_acc = Ok (inventory.clone());
	process.input
		.iter()
		.fold(original_acc, |acc_res: Result<Inventory, &'a str>, resource| {
			match acc_res {
				Ok (acc) => {
					let n_items = acc.get(&resource.name).ok_or("Unable to find resource in inventory")?;
					let mut new_acc = acc.clone();
					new_acc.insert(resource.name.clone(), *n_items - resource.quantity);
					Ok(acc)
				}
				err => err
			}
		})
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
