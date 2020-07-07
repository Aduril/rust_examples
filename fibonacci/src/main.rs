use std::io;

fn main() {
    let n = loop {
        println!("Which Fibonacci number should be displayed?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let to_convert: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if to_convert == 0 {continue};
        break to_convert;
    };

    let mut a = 0;
    let mut b = 1;
    let mut i = 0;
    while i < n {
        i = i + 1;
        let helper = a + b;
        a = b;
        b = helper;
    }

    println!(" The {}. Fibonacci number is {}", n, b);
}
