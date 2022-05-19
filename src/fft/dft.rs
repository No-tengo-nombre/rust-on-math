use crate::{Number, Complex, PI};

pub fn dft1<T: Number>(values: Vec<T>) -> Vec<Complex> {
    let length = values.len() as i32;
    let mut result = Vec::<Complex>::new();
    for k in 0..length {
        let mut temp = Complex::zero();
        for n in 0..length {
            temp += values[n as usize].to_complex() * (-Complex::j() * 2 * n * k * PI / length);
        }
        result.push(temp);
    }
    return result;
}

// pub fn dft1_f32(values: Vec<f32>) -> Vec<Complex> {
//     let length = values.len() as i32;
//     let mut result = Vec::<Complex>::new();
//     for k in 0..length {
//         let mut temp = Complex::zero();
//         for n in 0..length {
//             temp += values[n as usize].to_complex() * (-Complex::j() * 2 * n * k * PI / length).exp();
//         }
//         result.push(temp);
//     }
//     return result;
// }

// pub fn dft1_complex(values: Vec<Complex>) -> Vec<Complex> {
//     let length = values.len() as i32;
//     let mut result = Vec::<Complex>::new();
//     for k in 0..length {
//         let mut temp = Complex::zero();
//         for n in 0..length {
//             temp += values[n as usize] * (-Complex::j() * 2 * n * k * PI / length);
//         }
//         result.push(temp);
//     }
//     return result;
// }
