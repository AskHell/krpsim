use std::collections::{HashMap, BinaryHeap};

use super::{
    node::Node,
    ast::{Simulation},
};

#[derive(Debug)]
pub struct Solved {
    pub content: Vec<Node>,
}

pub fn algo(sim: Simulation) -> Result<Solved, String> {
    let mut closeset = Vec::<Node>::new();
    let mut openset = BinaryHeap::<Node>::new();

    let mut actual_node = Node::new(0, sim.inventory.clone(), Vec::new(), Vec::new(), 0, 0);
    let mut best_node = actual_node.clone();

    for _ in 0..10000 {
        println!("node H: {}", actual_node.h);
        println!("node TIME: {}", actual_node.time);
        closeset.push(actual_node.clone());

        let childs = actual_node.get_childs(&sim, closeset.len() - 1);

        println!("nchilds: {}", childs.len());

        for child in childs.into_iter() {
            openset.push(child);
        }

        actual_node = openset.pop().unwrap(); // Get best node !

        println!("poped node H: {}", actual_node.h);
        println!("");

        if actual_node.f > best_node.f {
            best_node = actual_node.clone();
        }
    }

    let mut final_node = best_node;
    let mut index = final_node.parent;
    let mut res = Solved { content: Vec::new() };

    while index != 0 {
        final_node = closeset.remove(index);
        index = final_node.parent;

        res.content.push(final_node);
    }

    res.content.reverse();

    Ok(res)
}
