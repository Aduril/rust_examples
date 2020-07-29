pub fn calc(integers: &Vec<i32>) -> f32 {
    let length = integers.len() as f32;
    let mut mean: f32 = 1.0;
    for integer in integers {
        mean *= (*integer as f32).powf(length.recip());
    }
    let rounded_result = (mean * 100.0).round() / 100.0;
    return rounded_result;
}
