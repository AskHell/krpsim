use std::cmp::Ordering;

use super::ast::{
    Simulation,
    Process,
};

use super::inventory::{
    Inventory,
    inventory_add,
    inventory_compare,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub parent: usize,
    pub inventory: Inventory,
    pub input: Vec<(Process, i32)>,
    pub output: Vec<(Process, i32)>,
    pub h: i32,
    pub time: i32,
    pub f: i32,
}

impl Node {
    pub fn new(
        parent: usize,
        inventory: Inventory,
        input: Vec<(Process, i32)>,
        output: Vec<(Process, i32)>,
        h: i32,
        time: i32
    ) -> Self {
        Node {
            parent,
            inventory,
            input,
            output,
            h,
            time,
            f: h - time,
        }
    }

    pub fn get_transversal_processes(&self) -> Vec<(Process, i32)> {
        self.output.iter()
            .filter_map(|p| {
                let mut is_in = false;

                for input in self.input.iter() {
                    if input == p {
                        is_in = true;
                        break;
                    }
                }
                match is_in {
                    true => None,
                    false => Some(p.clone())
                }
            }).collect()
    }

    pub fn separate_finished_processes(processes: &Vec<(Process, i32)>, time: i32) -> (Vec<(Process, i32)>, Vec<(Process, i32)>) {
        processes.iter()
            .fold((Vec::new(), Vec::new()), |(mut finished, mut active), (p, t)| {
                if *t <= time {
                    finished.push((p.clone(), *t))
                } else {
                    active.push((p.clone(), *t))
                }
                (finished, active)
            })
    }

    pub fn get_available_processes(inventory: &Inventory, simulation: &Simulation, time: i32) -> Vec<(Process, i32)> {
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

    fn get_possible_outputs_closure(
        mut acc: Vec<(Vec<(Process, i32)>, Inventory)>,
        mut actual: Vec<(Process, i32)>,
        inventory: Inventory,
        simulation: &Simulation,
        time: i32
    ) -> Vec<(Vec<(Process, i32)>, Inventory)> {
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
                    acc = Self::get_possible_outputs_closure(
                        acc,
                        actual.clone(),
                        process.apply_to(inventory.clone()),
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
    pub fn get_possible_outputs(inventory: &Inventory, simulation: &Simulation, time: i32) -> Vec<(Vec<(Process, i32)>, Inventory)> {
        Self::get_possible_outputs_closure(
            Vec::new(),
            Vec::new(),
            inventory.clone(),
            simulation,
            time
        )
    }

    /// Returns a list of all possible child Nodes (aka: every possible path to take) for the A* algorithme.
    pub fn get_childs(&self, simulation: &Simulation, id: usize) -> Vec<Self> {
        let time = self.output
            .iter()
            .map(|(_, t)| t.clone())
            .min()
            .unwrap_or(0);

        let (finished, input) = Self::separate_finished_processes(&self.output, time);

        let new_inventory = finished
            .into_iter()
            .fold(self.inventory.clone(), |acc, (p, _)| {
                inventory_add(&acc, &p.output)
            });

        let possible_output = Self::get_possible_outputs(&new_inventory, simulation, time);

        possible_output.into_iter().map(
            | (output, inv) | Self::new(
                id,
                inv,
                input.clone(),
                output,
                0,
                time
            ).apply_heuristic(self)
        ).collect()
    }

    fn apply_heuristic(mut self, old: &Self) -> Self {
        let h = self.output.iter().fold(0, |acc, (p, _)| acc + p.h) + old.h;

        self.h = h;
        self
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.h.cmp(&other.h)
    }
}
