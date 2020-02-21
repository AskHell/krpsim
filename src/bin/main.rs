use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

use krpsim::{
    parser::SimulationBuilderParser,
    ast::Simulation,
	solver::Solver
};

fn parse(content: String) -> Result<Simulation, String> {
    SimulationBuilderParser::new()
        .parse(&content)
        .map_err(|err| format!("{}", err))
        .map(|simbuilder| Simulation::from(simbuilder))
}

fn main() -> Result<(), ()> {
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
    let _delay = matches
        .value_of("DELAY")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let simulation = match File::open(file) {
        Ok(mut file) => {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();
            parse(content)
			.map(|simulation| {
				let solver = Solver::new(0.01, 10, 100, 10);
				match solver.solve(&simulation) {
					Ok (best_path) => {
						println!("{:?}", best_path);
					}
					Err (err) => {
						println!("{}", err);
					}
				}
			})
        },
        Err(error) => Err(format!("{}", error)),
    };

    simulation
        .map(|sim| {
            println!("{:#?}", sim);
        })
        .map_err(|err| {
            println!("{}", err);
        })
}
