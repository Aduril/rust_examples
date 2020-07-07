use std::io;
fn main() {
    loop {       
        println!("From Fahrenheit to Celsius (F) or from Celsius to Fahrenheit (C)?");
        let mut switch = String::new();
        io::stdin()
            .read_line(&mut switch)
            .expect("Failed to read line!");

        let switch = switch.trim();
        if switch == "F" {
            calc_fahrenheit_to_celsius();
            break;
        } else if switch == "C" {
            calc_celsius_to_fahrenheit();
            break;
        }
    }
}

fn calc_fahrenheit_to_celsius() {
    let to_convert = read_to_convert();
    let result = (to_convert - 32.0) * 5.0/9.0;
    println!("{}°F is about {}°C", to_convert, result);
}

fn calc_celsius_to_fahrenheit() {
    let to_convert = read_to_convert();
    let result: f32 = to_convert * 9.0/5.0 + 32.0;
    println!("{}°C is about {}°F", to_convert, result);
}

fn read_to_convert() -> f32 {
    loop {
        println!("What number should be converted?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let to_convert: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break to_convert;
    }
}
