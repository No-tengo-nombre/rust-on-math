use rom_rs::{arange, linspace, logspace};

#[test]
fn axis_linspace() {
    assert_eq!(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        linspace!(1, 10, 10),
    );
    assert_eq!(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        linspace!(1.0, 10.0, 10),
    );
    assert_eq!(
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        linspace!(0.0, 10.0, 11),
    );
}

#[test]
fn axis_logspace() {
    assert_eq!(vec![1.0, 10.0, 100.0, 1000.0], logspace!(0, 3, 4));
    assert_eq!(vec![2.0, 4.0, 8.0], logspace!(1, 3, 3, 2));
    assert_eq!(vec![9.0, 27.0, 81.0, 243.0], logspace!(2, 5, 4, 3));
}

#[test]
fn axis_arange() {
    let empty_vector = Vec::<f32>::new();

    assert_eq!(
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        arange!(10),
    );
    assert_eq!(vec![5.1, 6.1, 7.1, 8.1, 9.1], arange!(5.1, 10.1));
    assert_eq!(vec![5.1, 6.1, 7.1, 8.1, 9.1, 10.1], arange!(5.1, 10.2));
    assert_eq!(empty_vector, arange!(0));
    assert_eq!(empty_vector, arange!(2, -1));
    assert_eq!(vec![2.0, 1.0, 0.0, -1.0, -2.0], arange!(2, -3, -1));
}
