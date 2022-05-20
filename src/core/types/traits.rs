use crate::Complex;

pub trait Number {
    fn to_complex(&self) -> Complex;
    fn zero() -> Self;
}

impl Number for Complex {
    fn to_complex(&self) -> Complex {
        return *self;
    }

    fn zero() -> Self {
        return Complex::zero();
    }
}

impl Number for f32 {
    fn to_complex(&self) -> Complex {
        return Complex::new(*self, 0.0);
    }

    fn zero() -> Self {
        return 0.0;
    }
}

impl Number for i32 {
    fn to_complex(&self) -> Complex {
        return Complex::new(*self as f32, 0.0);
    }

    fn zero() -> Self {
        return 0;
    }
}
