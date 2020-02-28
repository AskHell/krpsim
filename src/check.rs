use super::inventory::Inventory;
use super::ast::{Process, Resource, Simulation};

pub struct Output {
	pub steps: Vec<String>
}

fn consume_resource<'a>(acc_res: Result<Inventory, &'a str>, resource: &Resource) -> Result<Inventory, &'a str> {
	match acc_res {
		Ok (acc) => {
			let n_items = acc.get(&resource.name).ok_or("Unable to find resource in inventory")?;
			let mut new_acc = acc.clone();
			if *n_items < resource.quantity {
				Err("Not enough available resources")
			} else {
				new_acc.insert(resource.name.clone(), *n_items - resource.quantity);
				Ok(new_acc)
			}
		}
		err => err
	}
}

pub fn consume_resources<'a>(input: &Vec<Resource>, inventory: Inventory) -> Result<Inventory, &'a str> {
	let original_acc = Ok(inventory);
	input
		.into_iter()
		.fold(original_acc, consume_resource)
}

pub fn produce_resources<'a>(output: &Vec<Resource>, inventory: Inventory) -> Result<Inventory, &'a str> {
	let original_acc = Ok(inventory);
	output
		.into_iter()
		.fold(original_acc, |acc_res: Result<Inventory, &'a str>, resource| {
			match acc_res {
				Ok (acc) => {
					let n_items = acc.get(&resource.name).unwrap_or(&0);
					let mut new_acc = acc.clone();
					new_acc.insert(resource.name.clone(), *n_items + resource.quantity);
					Ok(new_acc)
				}
				err => err
			}
		})
}

pub fn manage_resources<'a>(inventory: Inventory, process: &Process) -> Result <Inventory, &'a str> {
	consume_resources(&process.input, inventory)
		.and_then(|consumed_inventory| produce_resources(&process.output, consumed_inventory))
}

pub fn check<'a>(simulation: Simulation, output: Output) -> Result <Inventory, &'a str> {
	output.steps.iter()
		.fold(Ok(simulation.inventory.clone()), |inventory_res, step| {
			match inventory_res {
				Ok (inventory) => {
					let process = simulation.processes.get(step).ok_or("Unable to find step in simulation")?;
					manage_resources(inventory, process)
				},
				err => err
			}
		})
}
