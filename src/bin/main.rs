use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

use krpsim::{
    parser::SimulationBuilderParser,
    ast::{Simulation},
	solver::{solve, Production},
	utils::generalize_error,
	score::build_resource_map,
};

fn parse<'a>(content: String) -> Result<Simulation, String> {
    SimulationBuilderParser::new()
        .parse(&content)
        .map_err(|err| format!("{:?}", err))
        .map(|simbuilder| Simulation::from(simbuilder))
}

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
	let map = build_resource_map(&simulation);
	println!("{:?}", map);
	Err("nope".to_string())
}

fn main() {
	let result = krpsim();
	
	match result {
		Err (err) => println!("An error occurred: {:?}", err),
		Ok (best_path) => println!("{:?}", best_path)
	}
}

