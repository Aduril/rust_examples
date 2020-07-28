fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // println!("The 18th element is {}", &v[17]); // will panic

    let mut v2 = vec![1, 2, 3, 4, 5];

    v2.push(6);
    let first = &v2[0]; 
    // v2.push(7); // will not compile due to mutable borrow
    println!("The first element is: {}", first);
}
