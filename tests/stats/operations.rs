use rom_rs::{Complex, complex, mean, median};
use rom_rs::stats::mean_complex;

#[test]
fn operations_mean_reals() {
    assert_eq!(2.0, mean![2, 2.0, 2.00, 2]);
    assert_eq!(3.0, mean![1, 2, 3.0, 4.0, 5.0]);
    assert_eq!(3.0, mean![3, 5.0, 1, 4.0, 2.0]);
}

#[test]
fn operations_mean_complex() {
    let mut test_vec1 = Vec::<Complex>::new();
    test_vec1.push(Complex::one());
    test_vec1.push(Complex::j());
    test_vec1.push(complex!(-1, -1));
    assert_eq!(0, mean_complex(test_vec1));

    let mut test_vec2 = Vec::<Complex>::new();
    test_vec2.push(complex!(1, 1));
    test_vec2.push(complex!(2, 2));
    test_vec2.push(complex!(3, 3));
    assert_eq!(complex!(2, 2), mean_complex(test_vec2));
}

#[test]
fn operations_median() {
    assert_eq!(2.0, median![2, 2.0, 2.00, 2]);
    assert_eq!(3.0, median![1, 2, 3.0, 4, 5.00]);
    assert_eq!(3.0, median![3, 5.0, 1, 4.0, 2.0]);
    assert_eq!(3.5, median![1.0, 2, 3.0, 4, 5.00, 6]);
}
