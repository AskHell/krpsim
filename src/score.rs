use std::collections::HashMap;
use std::cmp::max;

use crate::ast;
use crate::ast::{Simulation};
use crate::solver::{Path};
use crate::simulate::simulate;

pub type Score = i32;

type Weight = usize;
type Frequency = usize;

#[derive(Debug)]
pub struct Resource {
	name: String,
	weight: Weight,
	frequency: Frequency,
}

type ResourceMap = HashMap<String, Resource>;


fn find_dependencies(simulation: &Simulation, resource_name: &String) -> Option<Vec<ast::Resource>> {
	let mut dependencies: Vec<ast::Resource> = vec![];

	for (_, process) in &simulation.processes {
		for resource in &process.output {
			if resource.name == *resource_name {
				for resource_needed in &process.input {
					if resource_needed.name != *resource_name {
						dependencies.push(resource_needed.clone())
					}
				}
			}
		}
	}
	if dependencies.is_empty() { None } else { Some(dependencies) }
}

fn update_resource_map(map: &mut ResourceMap, resource_name: String, weight: usize) {
	let default = Resource {
		name: "".to_string(),
		weight: 0,
		frequency: 0,
	};
	let existing_resource = map.get(&resource_name).unwrap_or(&default);
	let to_insert = Resource {
		name: resource_name.clone(),
		weight: max(existing_resource.weight, weight),
		frequency: existing_resource.frequency + 1,
	};
	map.insert(resource_name, to_insert);
}

fn dive_in(simulation: &Simulation, map: &mut ResourceMap, dependencies: &Vec<ast::Resource>) -> usize {
	let dependencie_names = dependencies.iter().map(|curr| {curr.name.clone()});
	for current_name in dependencie_names {
		match find_dependencies(simulation, &current_name) {
			Some (more_dep) => {
				let weight = 10 * dive_in(simulation, map, &more_dep);
				update_resource_map(map, current_name.clone(), weight);
			}
			None => {
				update_resource_map(map, current_name, 1);
			}
		}
	}
	1
}

pub fn build_resource_map(simulation: &Simulation) -> ResourceMap {
	let mut resource_map: ResourceMap = HashMap::new();

	for resource_name in &simulation.optimize {
		match find_dependencies(simulation, resource_name) {
			Some (dependencies) => {
				dive_in(simulation, &mut resource_map, &dependencies);
			}
			None => {}
		}
	}
	let max_weight: usize =
		resource_map
			.iter()
			.map(|(_, value)| { value.weight })
			.max()
			.unwrap_or(1);
	for resource_name in &simulation.optimize {
		update_resource_map(&mut resource_map, resource_name.clone(), max_weight * 10);
	}
	resource_map
}

pub struct Scorer {
	simulation: Simulation,
	resource_map: ResourceMap,
	time_weight: f32
}

impl Scorer {
	pub fn new(simulation: Simulation, time_weight: f32) -> Self {
		let resource_map = build_resource_map(&simulation);
		Self {
			simulation: simulation.clone(),
			resource_map,
			time_weight: if simulation.optimize_time { time_weight } else { 0. }
		}
	}

	pub fn score(&self, path: &Path) -> Score {
		let (inventory, duration) = simulate(&self.simulation, &path, self.simulation.optimize_time);
		let stock_score =
			inventory
			.into_iter()
			.fold(0, |score, (name, quantity)| {
				let resource = self.resource_map.get(&name).unwrap(); //TODO: protect
				let resource_score = resource.weight * resource.frequency * quantity;
				score + resource_score
			});
		let time_score = duration as f32 * self.time_weight;
		time_score.round() as Score + stock_score as Score
	}
}
