pub fn is_int(num: f32) -> bool {
    return num == num.floor();
}

pub fn i32_to_f32(vals: Vec<i32>) -> Vec<f32> {
    let mut result = Vec::<f32>::new();
    for i in vals {
        result.push(i as f32);
    }
    return result;
}
