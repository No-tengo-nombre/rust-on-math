use rom_rs::complex;

#[test]
fn complex_add() {
    assert_eq!(
        complex!(0, 0),
        complex!(-1, -1) + complex!(1, 1)
    );
    assert_eq!(
        complex!(0, 0),
        complex!(1, 1) + complex!(-1, -1)
    );
    assert_eq!(
        complex!(1, 1),
        complex!(1, 1) + complex!(0, 0)
    );

    assert_eq!(
        complex!(10, 1),
        complex!(8.7, 1) + 1.3
    );
}