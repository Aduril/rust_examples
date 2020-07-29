pub fn calc(integers: &Vec<i32>) -> f32 {
    let length = integers.len();
    match length % 2 {
        1 => integers[length / 2] as f32,
        0 => (integers[length / 2 - 1] + integers[length / 2]) as f32 / 2.0,
        _ => {
            println!("Something weird occurred when calculation the median");
            0.0
        }
    }
}
