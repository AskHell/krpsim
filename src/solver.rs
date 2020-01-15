use super::ast::{
    Simulation,
    Process,
    Inventory
};

fn max_of_iter<T: Ord>(mut i: impl Iterator<Item = T>) -> T {
    use std::cmp;
    let start = i.nth(0).unwrap();

    i.fold(start, |acc, item| cmp::max(acc, item))
}

struct Node {
    pub parent: usize,
    pub inventory: Inventory,
    pub input: Vec<(Process, u32)>
    pub ouput: Vec<(Process, u32)>,
    pub h: u32,
    pub time: u32,
}

impl Node {
    fn add_inventories() -> Inventory {

    }

    pub fn from_parent(parent: &Self) -> Self {
        let time = parent.ouput
            .into_iter()
            .map(|(p, t)| t)
            .min();

        let mut inventory = parent.inventory;
    }
}