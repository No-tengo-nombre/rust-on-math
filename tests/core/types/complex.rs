use rom_rs::complex;

#[test]
fn complex_add() {
    assert_eq!(complex!(0, 0), complex!(0, 0) + complex!(0, 0));
    assert_eq!(complex!(0, 0), complex!(-1, -1) + complex!(1, 1));
    assert_eq!(complex!(0, 0), complex!(1, 1) + complex!(-1, -1));
    assert_eq!(complex!(1, 1), complex!(1, 1) + complex!(0, 0));

    assert_eq!(complex!(0, 0), complex!(0, 0) + 0.0);
    assert_eq!(complex!(10, 1), complex!(8.7, 1) + 1.3);
    assert_eq!(complex!(10, 1), complex!(9, 1) + 1);
    assert_eq!(complex!(0, 0), complex!(-1, 0) + 1);
    assert_eq!(complex!(0, 0), complex!(0, 0) + 0);
}

#[test]
fn complex_sub() {
    assert_eq!(complex!(0, 0), complex!(1, 1) - complex!(1, 1));
    assert_eq!(complex!(1, 1), complex!(1, 1) - complex!(0, 0));
    assert_eq!(complex!(1, 1), complex!(1, 1) - complex!(0, 0));
}

#[test]
fn complex_mul() {
    assert_eq!(complex!(0, 0), complex!(5, 1) * complex!(0, 0));
    assert_eq!(complex!(5, 1), complex!(5, 1) * complex!(1, 0));
    assert_eq!(complex!(-1, 5), complex!(5, 1) * complex!(0, 1));
    assert_eq!(complex!(-4, 19), complex!(3, 2) * complex!(2, 5));
}

#[test]
fn complex_div() {
    assert_eq!(complex!(0, 0), complex!(0, 0) / complex!(1, 1));
    assert_eq!(complex!(1, 0), complex!(3, 4) / complex!(3, 4));
    assert_eq!(complex!(2, 1), complex!(8, 4) / 4);
    assert_eq!(complex!(2, 1), complex!(8, 4) / 4.0);
    assert_eq!(complex!(0.2, 0.1), complex!(8, 4) / 40);
}

#[test]
fn complex_eq() {
    assert_eq!(true, complex!(1, 1) == complex!(1, 1));
    assert_eq!(false, complex!(1, 1) == complex!(1, 0));
    assert_eq!(true, complex!(1, 0) == 1.0);
    assert_eq!(true, complex!(1, 0) == 1);
    assert_eq!(true, complex!(0, 0) == 0.0);
    assert_eq!(true, complex!(0, 0) == 0);
    assert_eq!(true, 1.0 == complex!(1, 0));
    assert_eq!(true, 1 == complex!(1, 0));
    assert_eq!(true, 0.0 == complex!(0, 0));
    assert_eq!(true, 0 == complex!(0, 0));

    assert_eq!(false, complex!(1, 1) != complex!(1, 1));
    assert_eq!(true, complex!(1, 1) != complex!(1, 0));
    assert_eq!(false, complex!(1, 0) != 1.0);
    assert_eq!(false, complex!(1, 0) != 1);
    assert_eq!(false, complex!(0, 0) != 0.0);
    assert_eq!(false, complex!(0, 0) != 0);
    assert_eq!(false, 1.0 != complex!(1, 0));
    assert_eq!(false, 1 != complex!(1, 0));
    assert_eq!(false, 0.0 != complex!(0, 0));
    assert_eq!(false, 0 != complex!(0, 0));
}
