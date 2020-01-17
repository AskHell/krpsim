use std::collections::HashMap;

use krpsim::node::*;
use krpsim::ast::*;

#[allow(dead_code)]
fn get_test_subject() -> (Vec<Simulation>, Vec<Process>, Vec<Inventory>) {
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

    (vec![sim_1, sim_2], processes, vec![inventory_1, inventory_2])
}

#[test]
fn get_available_processes() {
    let (sims, processes, inventories) = get_test_subject();

    let sim_1 = sims[0].clone();
    let sim_2 = sims[1].clone();

    let inventory_1 = inventories[0].clone();
    let inventory_2 = inventories[1].clone();

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

#[test]
fn get_possible_outputs() {
    let (sims, processes, inventories) = get_test_subject();

    let res_1 = Node::get_possible_outputs(
        inventories.get(0).unwrap(),
        sims.get(0).unwrap(),
        0
    );

    let inv_1 = inventory_sub_process(
        &inventories[0].clone(),
        processes.get(0).unwrap()
    );

    let inv_2 = inventory_sub_process(
        &inv_1.clone(),
        processes.get(0).unwrap()
    );

    let truth_1 = vec![
        (vec![], inventories[0].clone()),
        (vec![(processes[0].clone(), 1)], inv_1),
        (vec![(processes[0].clone(), 1), (processes[0].clone(), 1)], inv_2),
    ];

    assert_eq!(res_1, truth_1);
}
