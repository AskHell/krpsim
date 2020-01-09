use std::fs::File;
use std::io::Read;

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
    let file = "factorio.sim";

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
