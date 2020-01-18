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

impl From<(ProcessBuilder, &SimulationBuilder)> for Process {
    fn from((p, s): (ProcessBuilder, &SimulationBuilder)) -> Self {
        let h: u32 = 0;

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
            .map(|p| Process::from((p, &s)))
            .collect();

        Simulation {
            inventory: s.inventory,
            processes,
            optimize: s.optimize,
            optimize_time: s.optimize_time,
        }
    }
}
