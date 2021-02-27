fn gcd(mut n: i64, mut m: i64) -> i64 {
    if (n == 0 || m == 0) {
        1
    } else {
        while m != 0 {
            if m < n {
                std::mem::swap(&mut n, &mut m);
            }
            m = m % n;
        }
        n
    }
}

struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    fn new(n: i64, d: i64) -> Self {
        let g = gcd(n.abs(), d.abs());
        Self {
            numerator: n / g,
            denominator: d / g,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut n = 0;
        let mut d = 0;
        let mut flag = 0;
        for p in s.trim().split("/") {
            match flag {
                0 => {
                    n = match p.parse::<i64>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => panic!("Invalid number!"),
                    }
                }
                1 => {
                    d = match p.parse::<i64>() {
                        Result::Ok(num) => num,
                        Result::Err(_) => panic!("Invalid number!"),
                    }
                }
                _ => panic!("Invalid number!"),
            }
            flag += 1;
        }
        Self::new(n, d)
    }

    fn to_str(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    fn add(&self, rhs: &Rational) -> Self {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }

    fn sub(&self, rhs: &Rational) -> Self {
        Self::new(
            self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }

    fn mul(&self, rhs: &Rational) -> Self {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }

    fn div(&self, rhs: &Rational) -> Self {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }

    fn neg(&self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }

    fn less(&self, rhs: &Rational) -> bool {
        self.numerator * rhs.denominator < self.denominator * rhs.numerator
    }

    fn equal(&self, rhs: &Rational) -> bool {
        self.numerator * rhs.denominator == self.denominator * rhs.numerator
    }
}

#[test]
fn test_rational() {
    let r1 = Rational::new(1, 2);
    let r2 = Rational::new(1, 3);
    assert!(r1.add(&r2).equal(&Rational::new(5, 6)));
    assert!(r1.sub(&r2).equal(&Rational::new(1, 6)));
    assert!(r1.mul(&r2).equal(&Rational::new(1, 6)));
    assert!(r1.div(&r2).equal(&Rational::new(3, 2)));
    assert!(r1.neg().equal(&Rational::new(-1, 2)));
    assert!(!r1.less(&r2));
}

#[test]
fn test_rational_io() {
    let r = Rational::from_str("1/2");
    assert!(r.equal(&Rational::new(1, 2)));
    assert!(r.to_str() == "1/2");
}

fn main() {
    println!("Hello World!");
}
