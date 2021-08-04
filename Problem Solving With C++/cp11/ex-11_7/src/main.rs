#[macro_use]
extern crate scan_fmt;

use std::cmp;
use std::fmt;
use std::ops;
use std::str;

#[derive(Clone, Copy)]
struct Money {
    cents: i64,
}

impl Money {
    fn new() -> Self {
        Self { cents: 0 }
    }

    fn new_from_dollars_and_cents(dollars: i64, cents: i64) -> Self {
        Self {
            cents: dollars * 100 + cents,
        }
    }
}

impl ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            cents: self.cents + rhs.cents,
        }
    }
}

impl ops::Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            cents: self.cents - rhs.cents,
        }
    }
}

impl ops::Neg for Money {
    type Output = Self;

    fn neg(self) -> Self {
        Self { cents: -self.cents }
    }
}

impl cmp::PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.cents == other.cents
    }
}

impl cmp::PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cents.partial_cmp(&other.cents)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cents < 0 {
            write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
        } else {
            let cents = self.cents.abs();
            write!(f, "-${}.{:02}", cents / 100, cents.abs() % 100)
        }
    }
}

impl str::FromStr for Money {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scan_fmt!(s, "{}.{}", i64, i64) {
            Result::Ok((dollars, cents)) => {
                Result::Ok(Self::new_from_dollars_and_cents(dollars, cents))
            }
            Result::Err(_) => match scan_fmt!(s, "-{}.{}", i64, i64) {
                Result::Ok((dollars, cents)) => {
                    Result::Ok(Self::new_from_dollars_and_cents(-dollars, -cents))
                }
                Result::Err(_) => Result::Err(()),
            },
        }
    }
}

struct Check {
    no: u64,
    money: Money,
    cashed: bool,
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.no, self.money, self.cashed)
    }
}

impl str::FromStr for Check {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scan_fmt!(s, "{} {} {}", u64, String, bool) {
            Result::Ok((no, money_str, cashed)) => match Money::from_str(&money_str) {
                Result::Ok(money) => Result::Ok(Check {
                    no: no,
                    money: money,
                    cashed: cashed,
                }),
                Result::Err(_) => Result::Err(()),
            },
            Result::Err(_) => Result::Err(()),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
