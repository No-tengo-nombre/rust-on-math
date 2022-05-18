use crate::utils::is_int;

/// Indicates whether `num` is a power of 2
pub fn is_f32_base2(num: f32) -> bool {
    let num_log = num.log2();
    return is_int(num_log);
}

/// Indicates whether `num` is a power of `base`
pub fn is_f32_basen(num: f32, base: f32) -> bool {
    let num_log = num.log(base);
    return is_int(num_log);
}

/// Indicates whether `num` is a power of 2
pub fn is_i32_base2(num: i32) -> bool {
    let num_log = (num as f32).log2();
    return is_int(num_log);
}

/// Indicates whether `num` is a power of `base`
pub fn is_i32_basen(num: i32, base: f32) -> bool {
    let num_log = (num as f32).log(base);
    return is_int(num_log);
}
