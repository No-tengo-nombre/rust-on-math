use std::ops;
use std::fmt;

/// Complex number
#[derive(Debug, PartialEq, Clone)]
pub struct Complex {
    real: f32,
    imag: f32,
}

#[macro_export]
macro_rules! complex {
    ($x:expr, $y:expr) => {
        rom_rs::Complex::new($x as f32, $y as f32)
    };
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Complex {
        return Complex { real, imag };
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + j{}", self.real, self.imag)
        }
        else {
            write!(f, "{} - j{}", self.real, -self.imag)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//=====================================|| Addition ||============================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl ops::Add<Complex> for Complex {
    type Output = Complex;
    
    fn add(self, rhs: Complex) -> Complex {
        return Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        };
    }
}

impl ops::Add<f32> for Complex {
    type Output = Complex;

    fn add(self, rhs: f32) -> Complex {
        return Complex {
            real: self.real + rhs,
            imag: self.imag,
        };
    }
}

impl ops::Add<i32> for Complex {
    type Output = Complex;
    
    fn add(self, rhs: i32) -> Complex {
        return Complex {
            real: self.real + (rhs as f32),
            imag: self.imag,
        };
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//====================================|| Subtraction ||==========================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl ops::Sub<Complex> for Complex {
    type Output = Complex;
    
    fn sub(self, rhs: Complex) -> Complex {
        return Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        };
    }
}

impl ops::Sub<f32> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f32) -> Complex {
        return Complex {
            real: self.real - rhs,
            imag: self.imag,
        };
    }
}

impl ops::Sub<i32> for Complex {
    type Output = Complex;
    
    fn sub(self, rhs: i32) -> Complex {
        return Complex {
            real: self.real - (rhs as f32),
            imag: self.imag,
        };
    }
}
