use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

use krpsim::{
    parser::SimulationParser,
    ast::Simulation
};

fn parse(content: String) -> Result<Simulation, String> {
    SimulationParser::new()
        .parse(&content)
        .map_err(|err| format!("{}", err))
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
    let delay = matches
        .value_of("DELAY")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let simulation = match File::open(file) {
        Ok(mut file) => {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();
            parse(content)
        },
        Err(error) => Err(format!("{}", error)),
    };

    simulation
        .map(|sim| {})
        .map_err(|err| {
            println!("{}", err);
        })
}
