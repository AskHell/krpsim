use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

use krpsim::{
    ast::{parse},
	solver::{solve, Production, Path},
	utils::generalize_error,
	check::{Output, check}
};

// Todo: error if no delay ?
fn parse_args() -> Result<(String, usize), String> {
    let matches = App::new("krpsim")
        .author("Hugo Sabourin <hsabouri@student.42.fr>")
        .about("Process optimizer")
        .arg(Arg::with_name("FILE")
            .help("Input description file.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("DELAY")
            .help("Duration of the simulation")
            .required(false)
            .index(2))
        .get_matches();

    let delay = matches
        .value_of("DELAY")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let file_path = matches
        .value_of("FILE")
        .ok_or("Unable to open configuration file")?;
	
	Ok((file_path.to_string(), delay))
}

fn krpsim() -> Result<Production, String> {
	let (simulation_file_path, _delay) = parse_args()?;
	let mut simulation_file = File::open(simulation_file_path).map_err(generalize_error)?;
	let mut simulation_content = String::new();
	simulation_file.read_to_string(&mut simulation_content).unwrap();
	let simulation = parse(simulation_content)?;
	let result = solve(simulation.clone())?;
	let final_path:Vec<Path> = result.clone().into_iter().map(|(_, path)| { path }).collect();
	let flat_path = final_path.into_iter().fold(vec![], |acc, curr| { [&acc[..], &curr[..]].concat() });
	let output = Output { steps: flat_path };
	let final_inventory = check(simulation, output)?;
	Ok(result)
	
}

fn main() {
	let result = krpsim();
	
	match result {
		Err (err) => println!("An error occurred: {:?}", err),
		Ok (best_path) => {
			println!("{:?}", best_path);
		}
	}
}

