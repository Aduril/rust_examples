fn main() {
    // should work
    let mut str = String::from("Hello");
    str.push_str(" World");
    println!("Message: {}", str);
    // does not work
    // let mut str = "Hello";
    // str.push(" World");
    // println!("Message: {}", str);
    let mut s = String::from("hello");  // s comes into scope
    s.push_str("Hola!");
    takes_ownership(s);             // s's value moves into the function...
    // wont work
    // s.push_str("Hola!");
    // ... and so is no longer valid here
    
    let x = 5;                      // x comes into scope
    
    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(mut some_string: String) { // some_string comes into scope
    some_string.push_str(" World");
    println!("{}", some_string);
    some_string.push_str(" World");
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
