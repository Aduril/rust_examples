mod interactor;
mod calculus;
fn main() {
    let integers = interactor::request_integers();
    println!("Arithmetic mean : {}", calculus::arithmetic_mean::calc(&integers));
    println!("Geometric mean  : {}", calculus::geometric_mean::calc(&integers));
    println!("Median          : {}", calculus::median::calc(&integers));
    println!("Mode            : {}", stringify_vector(calculus::mode::calc(&integers)));
}

fn stringify_vector(integers: Vec<i32>) -> String {
    let mut str = String::from("");
    for integer in integers {
        str = format!("{} ,{}", str, integer);
    }
    return str.trim_start_matches(" ,").to_string();
}




