use std::collections::HashMap;

pub fn calc(integers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in integers {
        let counter = map.entry(*integer).or_insert(0);
        *counter += 1;
    }
    let mut max = 0;
    let mut mode: Vec<i32> = Vec::new();
    for (key, value) in map {
        if value > max {
            max = value;
            mode = Vec::new();
            mode.push(key);
        } else if value == max {
            mode.push(key)
        }
    }
    mode.sort();
    return mode;
}
