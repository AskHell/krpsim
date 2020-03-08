use std::collections::HashMap;

use super::inventory::{Inventory};
use super::parser::{SimulationBuilderParser};

#[derive(Debug, Clone)]
pub struct ProcessBuilder {
    pub name: String,
    pub input: Vec<Resource	>,
    pub output: Vec<Resource>,
    pub duration: usize
}

impl ProcessBuilder {
    pub fn new(name: String, input: Vec<Resource>, output: Vec<Resource>, duration: usize) -> Self {
        Self {
            name,
            input,
            output,
            duration
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
	pub name: String,
	pub quantity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub name: String,
    pub input: Vec<Resource>,
    pub output: Vec<Resource>,
    pub duration: usize
}

impl From<&ProcessBuilder> for Process {
    fn from(p: &ProcessBuilder) -> Self {
        Process {
            name: p.name.clone(),
            input: p.input.clone(),
            output: p.output.clone(),
            duration: p.duration,
        }
    }
}

impl Process {
    // pub fn apply_to(&self, inv: Inventory) -> Inventory {
    //     let mut res: Inventory = HashMap::new();

    //     for (key, value) in self.input.iter() {
    //         if let Some(has) = inv.get(key) {
    //             if has - value > 0 {
    //                 res.insert(key.clone(), has - value);
    //             }
    //         }
    //     }
    //     res
    // }

    pub fn new(name: String, input: Vec<Resource>, output: Vec<Resource>, duration: usize) -> Self {
        Self {
            name,
            input,
            output,
            duration
        }
    }
}

#[derive(Debug)]
pub struct SimulationBuilder {
    pub inventory: Inventory,
    pub processes: HashMap<String, ProcessBuilder>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

impl SimulationBuilder {
    pub fn default() -> Self {
        Self {
            inventory: HashMap::new(),
            processes: HashMap::new(),
            optimize: vec![],
            optimize_time: false,
        }
    }

    pub fn new(inventory: Inventory, processes: HashMap<String, ProcessBuilder>, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        Self {
            inventory,
            processes,
            optimize,
            optimize_time,
        }
    }

    pub fn add_inventory(mut self, name: String, quantity: usize) -> Self {
        self.inventory.insert(name, quantity);
        self
    }

    pub fn add_process(mut self, process: ProcessBuilder) -> Self {
        self.processes.insert(process.name.clone(), process);
        self
    }

    pub fn optimize(mut self, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        self.optimize = optimize;
        self.optimize_time = optimize_time;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulation {
    pub inventory: Inventory,
    pub processes: HashMap<String, Process>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

impl From<SimulationBuilder> for Simulation {
    fn from(s: SimulationBuilder) -> Self {
        let processes = s.processes
			.iter()
            .map(|p| {
                (p.0.clone(), Process::from(p.1))
            })
            .collect();

        Simulation {
            inventory: s.inventory,
            processes,
            optimize: s.optimize,
            optimize_time: s.optimize_time,
        }
    }
}

impl Simulation {
    pub fn new(inventory: Inventory, processes: HashMap<String, Process>, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        Self {
            inventory,
            processes,
            optimize,
            optimize_time,
        }
    }
}

pub fn parse<'a>(content: String) -> Result<Simulation, String> {
    SimulationBuilderParser::new()
        .parse(&content)
        .map_err(|err| format!("{:?}", err))
        .map(|simbuilder| Simulation::from(simbuilder))
}
