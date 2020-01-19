use std::collections::HashMap;
use super::inventory::{Inventory};

#[derive(Debug, Clone)]
pub struct ProcessBuilder {
    pub name: String,
    pub input: Inventory,
    pub output: Inventory,
    pub duration: u32,
}

impl ProcessBuilder {
    pub fn new(name: String, input: Inventory, output: Inventory, duration: u32) -> Self {
        Self {
            name,
            input,
            output,
            duration
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub name: String,
    pub input: Inventory,
    pub output: Inventory,
    pub duration: u32,
    pub h: u32,
}

impl From<(ProcessBuilder, u32)> for Process {
    fn from((p, h): (ProcessBuilder, u32)) -> Self {
        Process {
            name: p.name,
            input: p.input,
            output: p.output,
            duration: p.duration,
            h
        }
    }
}

impl Process {
    pub fn apply_to(&self, inv: Inventory) -> Inventory {
        let mut res: Inventory = HashMap::new();

        for (key, value) in self.input.iter() {
            if let Some(has) = inv.get(key) {
                if has - value > 0 {
                    res.insert(key.clone(), has - value);
                }
            }
        }
        res
    }

    pub fn new(name: String, input: Inventory, output: Inventory, duration: u32, h: u32) -> Self {
        Self {
            name,
            input,
            output,
            duration,
            h
        }
    }
}

#[derive(Debug)]
pub struct SimulationBuilder {
    pub inventory: Inventory,
    pub processes: Vec<ProcessBuilder>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

impl SimulationBuilder {
    pub fn default() -> Self {
        Self {
            inventory: HashMap::new(),
            processes: vec![],
            optimize: vec![],
            optimize_time: false,
        }
    }

    pub fn new(inventory: Inventory, processes: Vec<ProcessBuilder>, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        Self {
            inventory,
            processes,
            optimize,
            optimize_time,
        }
    }

    pub fn add_inventory(mut self, name: String, quantity: u32) -> Self {
        self.inventory.insert(name, quantity);
        self
    }

    pub fn add_process(mut self, process: ProcessBuilder) -> Self {
        self.processes.push(process);
        self
    }

    pub fn optimize(mut self, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        self.optimize = optimize;
        self.optimize_time = optimize_time;
        self
    }

    fn compute_heuristic_graph(&self, commodity: &String) -> u32 {
        let mut full_h = 0;

        for p in self.processes.iter() {
            let mut h = 0;

            if let Some(n) = p.output.get(commodity) {
                h += 1;
                for (key, value) in p.input.iter() {
                    h += self.compute_heuristic_graph(key) * value + n;
                }
            }
            full_h += h;
        }

        full_h
    }

    pub fn compute_heuristic_of_process(&self, p: &ProcessBuilder) -> u32 {
        let mut h = 0;

        for (key, value) in p.input.iter() {
            h += value * self.compute_heuristic_graph(key);
        }

        h
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulation {
    pub inventory: Inventory,
    pub processes: Vec<Process>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

impl From<SimulationBuilder> for Simulation {
    fn from(s: SimulationBuilder) -> Self {
        let processes = s.processes
            .clone()
            .into_iter()
            .map(|p| {
                let h = s.compute_heuristic_of_process(&p);

                Process::from((p, h))
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
    pub fn new(inventory: Inventory, processes: Vec<Process>, optimize: (Vec<String>, bool)) -> Self {
        let (optimize, optimize_time) = optimize;

        Self {
            inventory,
            processes,
            optimize,
            optimize_time,
        }
    }
}
