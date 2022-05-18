use std::fmt;
use std::ops;

/// Complex number
#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn j() -> Complex {
        return Complex {
            real: 0.0,
            imag: 1.0,
        };
    }

    pub fn one() -> Complex {
        return Complex {
            real: 1.0,
            imag: 0.0,
        };
    }

    pub fn real(&self) -> f32 {
        return self.real;
    }

    pub fn imag(&self) -> f32 {
        return self.imag;
    }

    pub fn len(&self) -> f32 {
        return self.len_squared().sqrt();
    }

    pub fn len_squared(&self) -> f32 {
        return self.real * self.real + self.imag * self.imag;
    }

    pub fn conj(&self) -> Complex {
        return Complex::new(self.real, -self.imag);
    }

    pub fn inv(&self) -> Complex {
        return self.conj() / self.len_squared();
    }

    pub fn exp(&self) -> Complex {
        return Complex::new(self.real.cos(), self.imag.sin());
    }

    pub fn cos(&self) -> Complex {
        let self_clone = self.clone();
        return ((Complex::j() * self_clone).exp() + (-Complex::j() * self_clone).exp()) / 2;
    }

    pub fn sin(&self) -> Complex {
        let self_clone = self.clone();
        return ((Complex::j() * self_clone).exp() - (-Complex::j() * self_clone).exp()) / Complex::new(0.0, 2.0);
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + j{}", self.real, self.imag)
        } else {
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

///////////////////////////////////////////////////////////////////////////////////////////////////
//===================================|| Multiplication ||========================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        return Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        };
    }
}

impl ops::Mul<f32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f32) -> Complex {
        return Complex {
            real: self.real * rhs,
            imag: self.imag * rhs,
        };
    }
}

impl ops::Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: i32) -> Complex {
        return Complex {
            real: self.real * (rhs as f32),
            imag: self.imag * (rhs as f32),
        };
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//======================================|| Division ||===========================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl ops::Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        return self.clone() * rhs.inv();
    }
}

impl ops::Div<f32> for Complex {
    type Output = Complex;

    fn div(self, rhs: f32) -> Complex {
        return Complex {
            real: self.real / rhs,
            imag: self.imag / rhs,
        };
    }
}

impl ops::Div<i32> for Complex {
    type Output = Complex;

    fn div(self, rhs: i32) -> Complex {
        return Complex {
            real: self.real / (rhs as f32),
            imag: self.imag / (rhs as f32),
        };
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//======================================|| Negative ||===========================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        return Complex::new(-self.real, -self.imag);
    }
}
