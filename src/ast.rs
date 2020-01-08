use std::collections::HashMap;
use std::hash::Hash;

/// Converts a Vec<(T, U)> in a HashMap<T, U>. Similar to Haskell function `fromList` in Data.Strict.Map module.
pub fn from_vec<T: Eq + Hash, U>(list: Vec<(T, U)>) -> HashMap<T, U> {
    let mut res: HashMap<T, U> = HashMap::new();

    for (t, u) in list.into_iter() {
        res.insert(t, u);
    }
    res
}

pub struct Process {
    name: String,
    input: HashMap<String, u32>,
    output: HashMap<String, u32>,
    duration: u32,
}

pub struct Simulation {
    inventory: HashMap<String, u32>,
    processes: HashMap<String, u32>,
    optimize: Vec<String>,
    optimize_time: bool,
}

