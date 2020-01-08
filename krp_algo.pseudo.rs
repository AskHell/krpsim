type Inventory = HashMap<String, i32>;

struct Process {
    name: String,
    input: HashMap<String, i32>,
    output: HashMap<String, i32>,
    time: f32,
}

struct Node {
    process: Process,
    inventory: Inventory, // Inventory DURING the process
    parent: i64, // ID of the parent in the closeset (-1 -> The original parent)
    h: f32, // heuristical value
    g: f32, // endtime, counts as distance to starting point
    f: f32, // g + h
}

let inventory : Inventory;
let processes : Vec<Process>;
let solution : Vec<Process> = vec![];

let open_set : Vec<Node>;
let close_set : Vec<Node>;
