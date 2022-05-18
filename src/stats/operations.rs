pub fn meanf(vals: Vec<f32>) -> f32 {
    let vals_len = vals.len() as f32;
    let mut result = 0.0;
    for i in vals {
        result += i;
    }
    return result / vals_len;
}

pub fn meani(vals: Vec<i32>) -> f32 {
    let vals_len = vals.len() as f32;
    let mut result = 0.0;
    for i in vals {
        result += i as f32;
    }
    return result / vals_len;
}
