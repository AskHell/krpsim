use std::fs::File;
use std::io::{Read, Result as IoResult};

use clap::{Arg, App};

use krpsim::{
    parser::SimulationBuilderParser,
    ast::Simulation,
	solver::solve,
	genetic_config_parser::parse_genetic_config,
};

fn parse(content: String) -> Result<Simulation, String> {
    SimulationBuilderParser::new()
        .parse(&content)
        .map_err(|err| format!("{}", err))
        .map(|simbuilder| Simulation::from(simbuilder))
}

fn io() -> IoResult<(File, usize, File)> {
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
    let file = matches
        .value_of("FILE")
        .unwrap();
    let delay = matches
        .value_of("DELAY")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
	
	let simulation_file = File::open(file)?;
	let genetic_config_file = File::open("generic_config.json")?;
	Ok((simulation_file, delay, genetic_config_file))
}

fn main() {
	match io() {
		Ok ((mut simulation_file, _delay, mut genetic_simulation_file)) => {
			let mut simulation_content = String::new();
			simulation_file.read_to_string(&mut simulation_content).unwrap();
			match parse(simulation_content) {
				Ok (simulation) => {
					let mut genetic_simulation_content = String::new();
					genetic_simulation_file.read_to_string(&mut genetic_simulation_content).unwrap();
					match parse_genetic_config(&genetic_simulation_content) {
						Ok (genetic_simulation_config) => {
							match solve(simulation, genetic_simulation_config) {
								Ok (best_path) => {
									println!("{:?}", best_path);
								}
								Err (err) => {
									println!("{}", err);
								}
							}
						},
						Err (err) => {
							println!("{}", err);
						}
					}
				},
				Err (err) => {
					println!("{}", err);
				}
			}
		},
        Err(err) => {
			println!("{}", err);
		}
	};
}
