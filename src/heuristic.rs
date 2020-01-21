use super::ast::Simulation;
use super::node::{Node};

pub fn time_based_heuristic(sim: &Simulation, node: &Node) -> i32 {
    node.get_transversal_processes()
    .iter()
    .fold(0, |acc, (p, t)| {
        let mut add_opti: i32 = 0;

        for (key, val) in p.output.iter() {
            for name in sim.optimize.iter() {
                if key == name {
                    add_opti += val;
                }
            }
        }
        acc + add_opti + t
    })
}
