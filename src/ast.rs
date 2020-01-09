use std::collections::HashMap;
use std::hash::Hash;

/// Converts a Vec<(T, U)> in a HashMap<T, U>
pub fn convert<T: Eq + Hash, U>(input: Vec<(T, U)>) -> HashMap<T, U> {
    let mut res: HashMap<T, U> = HashMap::new();

    for (t, u) in input.into_iter() {
        res.insert(t, u);
    }
    res
}

pub type Inventory = HashMap<String, u32>;

#[derive(Debug)]
pub struct Process {
    name: String,
    input: Inventory,
    output: Inventory,
    duration: u32,
}

impl Process {
    pub fn new(name: String, input: Inventory, output: Inventory, duration: u32) -> Self {
        Self {
            name,
            input,
            output,
            duration
        }
    }
}

#[derive(Debug)]
pub struct Simulation {
    inventory: Inventory,
    processes: Vec<Process>,
    optimize: Vec<String>,
    optimize_time: bool,
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
