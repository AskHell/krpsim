use std::collections::HashMap;
use std::cmp::max;

use crate::ast::{self, Simulation};
use crate::solver::{Path};
use crate::simulate::simulate;
use super::{
    Score,
    ScoreMap,
};

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

fn update_score_map(map: &mut ScoreMap, resource: &ast::Resource, weight: usize) {
	let existing_score = map.get(&resource.name).unwrap_or(&0);
	let resource_score = weight as Score * resource.quantity as Score;
	let score = max(existing_score, &resource_score);
	map.insert(resource.name.clone(), *score);
}

fn dive_in(simulation: &Simulation, map: &mut ScoreMap, visited: &mut HashMap<String, bool>, dependencies: &Vec<ast::Resource>) -> usize {
	for current_dependency in dependencies {
		match find_dependencies(simulation, &current_dependency.name) {
			Some (more_dep) => {
				let filtered_dep: Vec<ast::Resource> = more_dep.into_iter().filter(|d| { !visited.contains_key(&d.name) }).collect();
				for d in filtered_dep.clone() {
					visited.insert(d.name.clone(), true);
				}
				let weight = 10 * dive_in(simulation, map, visited, &filtered_dep);
				update_score_map(map, current_dependency, weight);
			}
			None => {
				update_score_map(map, current_dependency, 1);
			}
		}
	}
	1
}

pub fn build_score_map_leo(simulation: &Simulation, _weight_multiplier: usize) -> ScoreMap {
	let mut score_map: ScoreMap = HashMap::new();

	for resource_name in &simulation.optimize {
		match find_dependencies(simulation, resource_name) {
			Some (dependencies) => {
				dive_in(simulation, &mut score_map, &mut HashMap::new(), &dependencies);
			}
			None => {}
		}
	}
	for resource_name in &simulation.optimize {
		let resource = ast::Resource {
			name: resource_name.clone(),
			quantity: 1
		};
		// TODO: unmock value
		update_score_map(&mut score_map, &resource, 1000);
	}
	score_map
}

pub fn leo_score(simulation: &Simulation, score_map: &ScoreMap, time_weight: f32, path: &Path) -> Result<Score, String> {
	let (inventory, duration) = simulate(&simulation, &path, false)?;
	let stock_score =
		inventory
		.into_iter()
		.fold(0, |score, (name, _)| {
			score + *score_map.get(&name).unwrap_or(&0)
		});
	let time_score = duration as f32 * time_weight;
	let score = stock_score as Score - time_score.round() as Score;
	Ok(score)
}
