use std::cmp;
use std::cmp::PartialOrd;
use std::ops;

struct Rational {
    _numerator: i64,
    _denominator: i64,
}

impl Rational {
    fn numerator(&self) -> i64 {
        self._numerator
    }

    fn denominatio(&self) -> i64 {
        self._denominator
    }

    fn new() -> Self {
        Self {
            _numerator: 0,
            _denominator: 1,
        }
    }

    fn new_from_integer(n: i64) -> Self {
        Self {
            _numerator: n,
            _denominator: 1,
        }
    }

    fn new_from_rational(n: i64, d: i64) -> Self {
        let mut ret = Self {
            _numerator: n,
            _denominator: d,
        };
        ret._normalize();
        ret
    }

    fn _normalize(&mut self) {
        let negative = (self._numerator < 0) ^ (self._denominator < 0);
        let n = self._numerator.abs();
        let d = self._denominator.abs();
        let gcd = Self::_gcd(n, d);
        self._numerator = if negative { -n / gcd } else { n / gcd };
        self._denominator = d / gcd;
    }

    fn _gcd(m: i64, n: i64) -> i64 {
        if m > n {
            return Self::_gcd(m - n, n);
        }
        if m < n {
            return Self::_gcd(n - m, m);
        }
        m
    }
}

impl ops::Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self {
        let mut ret = Self {
            _numerator: self._numerator * rhs._denominator + rhs._numerator * self._denominator,
            _denominator: self._denominator * rhs._denominator,
        };
        ret._normalize();
        ret
    }
}

impl ops::Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self {
        let mut ret = Self {
            _numerator: self._numerator * rhs._denominator - rhs._numerator * self._denominator,
            _denominator: self._denominator * rhs._denominator,
        };
        ret._normalize();
        ret
    }
}

impl ops::Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self {
        let mut ret = Self {
            _numerator: self._numerator * rhs._numerator,
            _denominator: self._denominator * rhs._denominator,
        };
        ret._normalize();
        ret
    }
}

impl ops::Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self {
        let mut ret = Self {
            _numerator: self._numerator * rhs._denominator,
            _denominator: self._denominator * rhs._numerator,
        };
        ret._normalize();
        ret
    }
}

impl cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        return self._numerator == other._numerator && self._denominator == other._denominator;
    }
}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        return Option::Some(
            (self._numerator * other._denominator).cmp(&(self._denominator * other._numerator)),
        );
    }
}

fn main() {
    println!("Hello, world!");
}
