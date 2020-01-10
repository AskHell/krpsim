use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

mod parser;
mod ast;

use ast::Simulation;

use parser::SimulationParser;

fn parse(content: String) {
    let parser = SimulationParser::new().parse(&content);

    match parser {
        Ok(sim) => { println!("{:#?}", sim); },
        Err(err) => { println!("{}", err); }
    }
}

fn main() {
    let matches = App::new("krpsim")
        .author("Hugo Sabourin <hsabouri@student.42.fr>")
        .about("Process optimizer")
        .arg(Arg::with_name("FILE")
            .help("Input description file.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("DELAY")
            .help("Time of the simulation")
            .required(false)
            .index(2))
        .get_matches();

    let file = matches.value_of("FILE").unwrap();
    let delay = matches.value_of("DELAY").unwrap_or("0").parse().unwrap_or(0);

    match File::open(file) {
        Ok(mut file) => {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();

            println!("{}", content);

            parse(content);
        },
        Err(error) => {
            println!("{}", error);
        },
    }
}
