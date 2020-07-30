use std::io;
use std::collections::HashMap;

const HELP_TEXT = "
Commands:
add_department <department_name>
remove_department <department_name>
add_user <user> to <department>
remove_user <user> to <department>
list
exit
"

fn main() {
    println!("Hello, what would you like to to?");
    print_help();
    let mut company = HashMap::new();
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

    }
}

fn print_help() {
    println!(HELP_TEXT);
}
