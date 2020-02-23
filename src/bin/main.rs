use std::fs::File;
use std::io::{Read, Result as IoResult};

use clap::{Arg, App};

use krpsim::{
    parser::SimulationBuilderParser,
    ast::Simulation,
	solver::solve,
	genetic_config_parser::parse_genetic_config,
};

enum Algorithm {
	Genetic,
	Simplex
}

fn parse<'a>(content: String) -> Result<Simulation, &'a str> {
    SimulationBuilderParser::new()
        .parse(&content)
        .map_err(|err| { println!("{:?}", err); "Parse error" })
        .map(|simbuilder| Simulation::from(simbuilder))
}

// Todo: handle flags ?
// Todo: error if no delay ?
fn parse_args<'a>() -> Result<(String, usize), &'a str> {
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

// TODO: unmock
fn get_algorithm(_simulation: &Simulation) -> Algorithm {
	Algorithm::Genetic
}

// Handle file openings (simulation description and genetic config)
fn io(file: &str) -> IoResult<(File, File)> {
	let simulation_file = File::open(file)?;
	let genetic_config_file = File::open("generic_config.json")?;
	Ok((simulation_file, genetic_config_file))
}

fn main() {
	let result = parse_args()
	.and_then(|(simulation_file_path, _delay)| {
		match io(&simulation_file_path) {
			Ok (res) => { Ok(res) },
			Err (err) => { println!("{}", err); Err("IO Error") }
		}
	})
	.and_then(|(mut simulation_file, mut genetic_config_file)| {
		// TODO: Check for algorithm to choose (only genetic available now)
		let mut simulation_content = String::new();
		simulation_file.read_to_string(&mut simulation_content).unwrap();
		let simulation = parse(simulation_content)?;
		let algorithm = get_algorithm(&simulation);
		let solve = match algorithm {
			Algorithm::Genetic => {
				let mut genetic_config_content = String::new();
				genetic_config_file.read_to_string(&mut genetic_config_content).unwrap();
				let genetic_config = parse_genetic_config(genetic_config_content)?;
				solve(simulation, genetic_config)
			},
			Algorithm::Simplex => Err ("Simplex not implemented yet")
		};
		solve
	});
	
	match result {
		Err (err) => println!("An error occurred: {:?}", err),
		Ok (best_path) => println!("{:?}", best_path)
	}
}

