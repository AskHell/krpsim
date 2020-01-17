use std::collections::HashMap;

use super::ast::{
    Simulation,
    Process,
    Inventory,
    inventory_add,
    inventory_compare,
    inventory_sub_process
};

fn max_of_iter<T: Ord>(mut i: impl Iterator<Item = T>) -> T {
    use std::cmp;
    let start = i.nth(0).unwrap();

    i.fold(start, |acc, item| cmp::max(acc, item))
}

pub struct Node {
    pub parent: usize,
    pub inventory: Inventory,
    pub input: Vec<(Process, u32)>,
    pub output: Vec<(Process, u32)>,
    pub h: u32,
    pub time: u32,
}

impl Node {
    fn separate_processes(processes: Vec<(Process, u32)>, time: u32) -> (Vec<Process>, Vec<Process>) {
        processes.into_iter()
            .fold((Vec::<Process>::new(), Vec::<Process>::new()), |(mut finished, mut active), (p, t)| {
                if t <= time {
                    finished.push(p)
                } else {
                    active.push(p)
                }
                (finished, active)
            })
    }

    pub fn get_available_processes(inventory: &Inventory, simulation: &Simulation, time: u32) -> Vec<(Process, u32)> {
        simulation.processes.clone().into_iter().filter_map(|p| {
            match inventory_compare(&p.input, inventory) {
                true => Some(p),
                false => None,
            }
        })
        .map(|p| {
            let end = p.duration + time;

            (p, end)
        })
        .filter_map(|(p, t)| {
            Some((p, t)) // TODO: Is end-time over simulation duration
        })
        .collect()
    }

    fn get_possible_output_closure(
        mut acc: Vec<(Vec<(Process, u32)>, Inventory)>,
        mut actual: Vec<(Process, u32)>,
        inventory: Inventory,
        simulation: &Simulation,
        time: u32
    ) -> Vec<(Vec<(Process, u32)>, Inventory)> {
        let available = Self::get_available_processes(&inventory, simulation, time);

        acc.push((actual.clone(), inventory.clone()));

        if available.len() > 0 {
            for (process, process_end) in available.into_iter() {
                let last = actual.pop();

                let explore = match last {
                    Some((act_p, act_t)) => if process_end <= act_t {
                        actual.push((act_p, act_t));
                        true
                    } else {
                        actual.push((act_p, act_t));
                        false
                    },
                    None => {
                        true
                    }
                };
                if explore {
                    actual.push((process.clone(), process_end));
                    acc = Self::get_possible_output_closure(
                        acc,
                        actual.clone(),
                        inventory_sub_process(&inventory, &process),
                        simulation,
                        time
                    );
                }
            }
        }
        acc
    }

    /// Returns a restrained list of all unique possible combination of processes.
    /// Unique in such a way that if a combination `a:(2, 1, 1)` exists, a similar combination but ordered differently is not possible (ex: `b:(1, 2, 1)`)
    /// The process used to avoid similar combination returns a sorted list as a side effect.
    pub fn get_possible_output(inventory: &Inventory, simulation: &Simulation, time: u32) -> Vec<(Vec<(Process, u32)>, Inventory)> {
        Self::get_possible_output_closure(
            Vec::new(),
            Vec::new(),
            inventory.clone(),
            simulation,
            time
        )
    }

    /* // Hidden for commit and tests
    pub fn get_childs(&self, simulation: &Simulation) -> Vec<Self> {
        let time = self.output
            .into_iter()
            .map(|(p, t)| t)
            .min()
            .unwrap();

        let (finished, input) = Self::separate_processes(self.output, time);

        let new_inventory = finished
            .into_iter()
            .fold(self.inventory.clone(), |acc, p| {
                inventory_add(&acc, &p.output)
            });

        let possible_output = Self::get_possible_output(new_inventory, simulation, time);
    }
    */
}

#[test]
fn test_get_available_processes() {
    let mut processes: Vec<Process> = Vec::new();

    let mut inventory_1: Inventory = HashMap::new();
    inventory_1.insert(String::from("machine"), 2);
    inventory_1.insert(String::from("metal"), 10);
    inventory_1.insert(String::from("gear"), 0);

    // Gear process
    let mut input: Inventory = HashMap::new();
    input.insert(String::from("machine"), 1);
    input.insert(String::from("metal"), 2);

    let mut output: Inventory = HashMap::new();
    output.insert(String::from("machine"), 1);
    output.insert(String::from("gear"), 1);

    processes.push(Process::new(String::from("do_gear"), input, output, 1));

    let mut inventory_2: Inventory = HashMap::new();
    inventory_2.insert(String::from("machine"), 2);
    inventory_2.insert(String::from("metal"), 20);
    inventory_2.insert(String::from("gear"), 4);

    // Science process
    let mut input: Inventory = HashMap::new();
    input.insert(String::from("machine"), 1);
    input.insert(String::from("metal"), 2);
    input.insert(String::from("gear"), 1);

    let mut output: Inventory = HashMap::new();
    output.insert(String::from("machine"), 1);
    output.insert(String::from("science"), 1);

    processes.push(Process::new(String::from("do_science"), input, output, 5));

    let sim_1 = Simulation::new(inventory_1.clone(), processes.clone(), (vec![], false));
    let sim_2 = Simulation::new(inventory_2.clone(), processes.clone(), (vec![], false));

    let res_1 = Node::get_available_processes(&inventory_1, &sim_1, 0);
    let res_2 = Node::get_available_processes(&inventory_2, &sim_2, 0);

    let valid_res_1 = vec![
        (processes[0].clone(), 1),
    ];

    let valid_res_2 = vec![
        (processes[0].clone(), 1),
        (processes[1].clone(), 5),
    ];

    println!("Test of gear process");
    assert_eq!(res_1, valid_res_1);
    println!("Test of science process");
    assert_eq!(res_2, valid_res_2);
}
