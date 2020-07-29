
use std::io;

pub fn request_integers() -> Vec<i32> {
    'outer: loop {
        let mut user_input = String::new();
        println!("Please give a space separated list of integers!");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");
        let string_vector: Vec<&str> = user_input.split(" ").collect();
        let mut int_vector: Vec<i32> = Vec::new();
        for value in string_vector {
            let trimmed_value = value.trim(); 
            if is_string_numeric(trimmed_value) {
                match trimmed_value.parse::<i32>() {
                    Ok(num) => int_vector.push(num),
                    Err(err) => {
                        println!("Error while parsing number '{}', reason: {}!", trimmed_value, err);
                        continue 'outer;
                    },
                }
            } else {
                continue 'outer;
            }
        }
        int_vector.sort();
        break int_vector;
    }   
}

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
