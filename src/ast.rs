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

pub fn inventory_add(left: &Inventory, right: &Inventory) -> Inventory {
    let mut res: Inventory = HashMap::new();

    for (key, value) in left.into_iter() {
        res.insert(key.clone(), match right.get(key) {
            Some(right_value) => value + right_value,
            None => value.clone()
        });
    }
    res
}

pub fn inventory_sub_process(inventory: &Inventory, process: &Process) -> Inventory {
    let mut res: Inventory = HashMap::new();

    for (key, value) in process.input.iter() {
        if let Some(has) = inventory.get(key) {
            if has - value > 0 {
                res.insert(key.clone(), has - value);
            }
        }
    }
    res
}

pub fn inventory_compare(left: &Inventory, right: &Inventory) -> bool {
    left.iter().fold(true, |acc, (key, value)| {
        if acc == false {false}
        else {
            match right.get(key) {
                Some(we_got) => we_got >= value,
                None => false
            }
        }
    })
}

#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub input: Inventory,
    pub output: Inventory,
    pub duration: u32,
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
    pub inventory: Inventory,
    pub processes: Vec<Process>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

impl Simulation {
    pub fn default() -> Self {
        Self {
            inventory: HashMap::new(),
            processes: vec![],
            optimize: vec![],
            optimize_time: false,
        }
    }

    pub fn new(inventory: Inventory, processes: Vec<Process>, optimize: (Vec<String>, bool)) -> Self {
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

    pub fn add_process(mut self, process: Process) -> Self {
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
