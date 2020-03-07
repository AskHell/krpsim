use std::collections::HashMap;
use std::cmp::max;

use crate::ast;
use crate::ast::{Simulation};
use crate::solver::{Path};
use crate::simulate::simulate;

pub type Score = i32;

type Weight = usize;
type Frequency = usize;

#[derive(Debug, Clone)]
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

fn update_resource_map(map: &mut ResourceMap, resource: &ast::Resource, weight: usize, default: Resource) {
	let existing_resource = map.get(&resource.name).unwrap_or(&default);
	let to_insert = Resource {
		name: resource.name.clone(),
		weight: max(existing_resource.weight, weight),
		frequency: existing_resource.frequency + 1,
	};
	map.insert(resource.name.clone(), to_insert);
}

fn dive_in(simulation: &Simulation, map: &mut ResourceMap, dependencies: &Vec<ast::Resource>, default_resource: &Resource) -> usize {
	for current_dependency in dependencies {
		match find_dependencies(simulation, &current_dependency.name) {
			Some (more_dep) => {
				let weight = 10 * dive_in(simulation, map, &more_dep, default_resource);
				update_resource_map(map, current_dependency, weight, default_resource.clone());
			}
			None => {
				update_resource_map(map, current_dependency, 1, default_resource.clone());
			}
		}
	}
	1
}

pub fn build_resource_map(simulation: &Simulation, default_resource: &Resource) -> ResourceMap {
	let mut resource_map: ResourceMap = HashMap::new();

	for resource_name in &simulation.optimize {
		match find_dependencies(simulation, resource_name) {
			Some (dependencies) => {
				dive_in(simulation, &mut resource_map, &dependencies, default_resource);
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
		let resource = ast::Resource {
			name: resource_name.clone(),
			quantity: 1
		};
		update_resource_map(&mut resource_map, &resource, max_weight * 10, default_resource.clone());
	}
	resource_map
}

pub struct Scorer {
	simulation: Simulation,
	resource_map: ResourceMap,
	time_weight: f32,
	default_resource: Resource
}

impl Scorer {
	pub fn new(simulation: Simulation, time_weight: f32) -> Self {
		let default_resource = Resource {
			name: "".to_string(),
			weight: 0,
			frequency: 0,
		};
		let resource_map = build_resource_map(&simulation, &default_resource);
		Self {
			simulation: simulation.clone(),
			resource_map,
			time_weight: if simulation.optimize_time { time_weight } else { 0. },
			default_resource
		}
	}

	pub fn score(&self, path: &Path) -> Result<Score, String> {
		// let (inventory, duration) = simulate(&self.simulation, &path, self.simulation.optimize_time)?;
		let (inventory, duration) = simulate(&self.simulation, &path, false)?;
		let stock_score =
			inventory
			.into_iter()
			.fold(0, |score, (name, quantity)| {
				let default = self.default_resource.clone();
				let resource = self.resource_map.get(&name).unwrap_or(&default);
				let resource_score = resource.weight * resource.frequency * quantity;
				score + resource_score
			});
		// let time_score = duration as f32 * self.time_weight;
		// let score = time_score.round() as Score + stock_score as Score;
		let score = stock_score as Score;
		Ok(score)
	}
}
