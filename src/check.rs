use std::collections::HashMap;

use super::inventory::Inventory;
use super::ast::{Process, Resource, Simulation};

pub struct Output {
	pub steps: Vec<String>
}

fn consume_resource<'a>(acc_res: Result<Inventory, String>, resource: &Resource) -> Result<Inventory, String> {
	match acc_res {
		Ok (acc) => {
			let n_items = acc.get(&resource.name).ok_or("Unable to find resource in inventory".to_string())?;
			let mut new_acc = acc.clone();
			if *n_items < resource.quantity {
				Err("Not enough available resources".to_string())
			} else {
				new_acc.insert(resource.name.clone(), *n_items - resource.quantity);
				Ok(new_acc)
			}
		}
		err => err
	}
}

pub fn consume_resources<'a>(input: &Vec<Resource>, inventory: Inventory) -> Result<Inventory, String> {
	let original_acc = Ok(inventory);
	input
		.into_iter()
		.fold(original_acc, consume_resource)
}

pub fn produce_resources<'a>(output: &Vec<Resource>, inventory: Inventory) -> Result<Inventory, String> {
	let original_acc = Ok(inventory);
	output
		.into_iter()
		.fold(original_acc, |acc_res: Result<Inventory, String>, resource| {
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

pub fn manage_resources<'a>(inventory: Inventory, process: &Process) -> Result <Inventory, String> {
	let mut hash_ok: Inventory = HashMap::new();
	hash_ok.insert("fond".to_string(), 1);
	hash_ok.insert("etagere".to_string(), 3);
	hash_ok.insert("montant".to_string(), 1);
	hash_ok.insert("planche".to_string(), 1);
	
	consume_resources(&process.input, inventory)
		.and_then(|consumed_inventory| {
			produce_resources(&process.output, consumed_inventory)
		})
}

pub fn check<'a>(simulation: Simulation, output: Output) -> Result <Inventory, String> {
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
