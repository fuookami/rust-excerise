use std::cmp;
use std::cmp::PartialOrd;
use std::ops;

pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    pub fn denominatio(&self) -> i64 {
        self.denominator
    }

    pub fn new() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    pub fn new_from_integer(n: i64) -> Self {
        Self {
            numerator: n,
            denominator: 1,
        }
    }

    pub fn new_from_rational(n: i64, d: i64) -> Self {
        let mut ret = Self {
            numerator: n,
            denominator: d,
        };
        ret.normalize();
        ret
    }

    fn normalize(&mut self) {
        let negative = (self.numerator < 0) ^ (self.denominator < 0);
        let n = self.numerator.abs();
        let d = self.denominator.abs();
        let gcd = Self::gcd(n, d);
        self.numerator = if negative { -n / gcd } else { n / gcd };
        self.denominator = d / gcd;
    }

    fn gcd(m: i64, n: i64) -> i64 {
        if m > n {
            return Self::gcd(m - n, n);
        }
        if m < n {
            return Self::gcd(n - m, m);
        }
        m
    }
}

impl ops::Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self {
        let mut ret = Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        ret.normalize();
        ret
    }
}

impl ops::Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self {
        let mut ret = Self {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        ret.normalize();
        ret
    }
}

impl ops::Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self {
        let mut ret = Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        };
        ret.normalize();
        ret
    }
}

impl ops::Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self {
        let mut ret = Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        };
        ret.normalize();
        ret
    }
}

impl cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        return self.numerator == other.numerator && self.denominator == other.denominator;
    }
}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        return Option::Some(
            (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator)),
        );
    }
}
