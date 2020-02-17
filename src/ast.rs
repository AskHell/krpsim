use std::collections::HashMap;
use super::inventory::{Inventory};

#[derive(Debug, Clone)]
pub struct ProcessBuilder {
    pub name: String,
    pub input: Vec<Resource>,
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

    // fn compute_heuristic_graph(&self, commodity: &String, tabs: usize) -> usize {
    //     let space = (0..tabs).fold(String::new(), |acc, _| format!("    {}", acc));
    //     let mut full_h = 0;

    //     println!("{}{}:", space, commodity);
    //     for p in self.processes.iter() {
    //         let mut h = 0;

    //         if let Some(n) = p.output.get(commodity) {
    //             if p.input.get(commodity).is_none() { // ignore if commodity is in both input and output
    //                 println!("    {}{}:", space, p.name);
    //                 h += n;
    //                 for (key, value) in p.input.iter() {
    //                     h += value * self.compute_heuristic_graph(key, tabs + 1);
    //                 }
    //             }
    //         }
    //         full_h += h;
    //     }

    //     println!("{}|-> {}", space, full_h);
    //     full_h
    // }

    // pub fn compute_heuristic_of_process(&self, p: &ProcessBuilder) -> usize {
    //     let mut h = 0;
    //     println!("{}:", p.name);

    //     for (key, value) in p.input.iter() {
    //         h += value * self.compute_heuristic_graph(key, 1);
    //     }

    //     if h == 0 { h = 1}

    //     h
    // }
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
