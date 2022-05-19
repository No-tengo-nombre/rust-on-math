use crate::Complex;

pub fn mean_f32(vals: Vec<f32>) -> f32 {
    let vals_len = vals.len() as f32;
    let mut result = 0.0;
    for i in vals {
        result += i;
    }
    return result / vals_len;
}

pub fn mean_complex(vals: Vec<Complex>) -> Complex {
    let vals_len = vals.len() as f32;
    let mut result = Complex::zero();
    for i in vals {
        result += i;
    }
    return result / vals_len;
}

pub fn median_f32(vals: Vec<f32>) -> f32 {
    let mut vals_sorted = vals.clone();
    vals_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let vals_len = vals_sorted.len() as i32;

    if vals_len % 2 == 0 {
        let i_high = (vals_len / 2) as usize;
        let i_low = i_high - 1;
        return (vals_sorted[i_high] + vals_sorted[i_low]) / 2.0;
    } else {
        return vals_sorted[((vals_len as f32) / 2.0).floor() as usize];
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//======================================|| Macros ||=============================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! mean {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x as f32);
        )*
        rom_rs::stats::mean_f32(temp_vec)
    }};
}

#[macro_export]
macro_rules! median {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x as f32);
        )*
        rom_rs::stats::median_f32(temp_vec)
    }};
}
