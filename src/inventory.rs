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

pub type Inventory = HashMap<String, i32>;

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
