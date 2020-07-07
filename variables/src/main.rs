fn main() {
    let x = 6;
    println!("x is {}", x);
    let x = 3.0/2.9;
    println!("x is {}", x);
    let a = [1,2,3,4,5,6];
    let index = 5;
    let element = a[index];
    println!("value is {}", element);
    function_one();
    function_two(element);
    let new_element = plus_one(element);
    println!("value of new element {}", new_element);
    let mut counter = 0;
    let result = loop {
        counter += 1;       
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}", result);
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn function_one() {
    println!("Another one bites the dust!");
}

fn function_two(x: i32) {
    println!("Another x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
