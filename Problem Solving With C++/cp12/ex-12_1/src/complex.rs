use std::cmp;
use std::fmt;
use std::ops;
use std::str::FromStr;

pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn new() -> Self {
        Self {
            real: 0.,
            imaginary: 0.,
        }
    }

    pub fn new_from_real(real: f64) -> Self {
        Self {
            real: real,
            imaginary: 0.,
        }
    }

    pub fn new_from_complex(real: f64, imaginary: f64) -> Self {
        Self {
            real: real,
            imaginary: imaginary,
        }
    }
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl cmp::PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        return self.real == other.real && self.imaginary == other.imaginary;
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {} i", self.real, self.imaginary)
    }
}

impl FromStr for Complex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scan_fmt!(s, "{} + {} i", f64, f64) {
            Result::Ok((real, imaginary)) => Result::Ok(Self::new_from_complex(real, imaginary)),
            Result::Err(_) => Result::Err(()),
        }
    }
}
