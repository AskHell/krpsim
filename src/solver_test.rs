use std::fs::File;
use std::io::Read;

use crate::utils::generalize_error;
use crate::ast::{Simulation, parse};
use crate::solver::{batchify, Batch};

fn test_provider(simulation_file_path: String) -> Result<Simulation, String> {
	let mut simulation_file = File::open(simulation_file_path).map_err(generalize_error)?;
	let mut simulation_content = String::new();
	simulation_file.read_to_string(&mut simulation_content).unwrap();
	parse(simulation_content)
}

#[test]
fn batchify_happy_path() {
	let processes: Vec<String> = vec![
		"do_montant".to_string(),
		"do_montant".to_string(),
		"do_fond".to_string(),
		"do_etagere".to_string(),
		"do_etagere".to_string(),
		"do_etagere".to_string(),
		"do_armoire_ikea".to_string(),
	];
	let expected: Vec<Batch> = vec![
		(20, vec![
			"do_montant".to_string(),
			"do_montant".to_string(),
			"do_fond".to_string(),
			"do_etagere".to_string(),
			"do_etagere".to_string(),
			"do_etagere".to_string(),
		]),
		(30, vec![
			"do_armoire_ikea".to_string(),
		])
	];
	match test_provider("ressources/ikea".to_string()) {
		Ok (simulation) => {
			let res = batchify(&simulation, processes);
			assert_eq!(res, Ok(expected))
		}
		Err (err) => { println!("An error occured: {}", err) }
	}
}
