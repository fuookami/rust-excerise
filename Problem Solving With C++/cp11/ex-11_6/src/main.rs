use std::clone;
use std::cmp::PartialOrd;
use std::ops;

struct Polynomial {
    coefficients: Vec<(usize, f64)>,
}

impl Polynomial {
    fn new() -> Self {
        Self {
            coefficients: vec![(0, 0.)],
        }
    }

    fn new_from_coefficients(coefficients: Vec<(usize, f64)>) -> Self {
        let mut c = Vec::new();
        for i in 0..coefficients.len() {
            if i == 0 || coefficients[i].1.abs() >= 1e-7 {
                c.push((coefficients[i].0, coefficients[i].1));
            }
        }
        Self { coefficients: c }
    }

    fn new_from_coefficient_list(coefficient_list: Vec<f64>) -> Self {
        let mut coefficients = Vec::new();
        for i in 0..coefficients.len() {
            if i == 0 || coefficient_list[i].abs() >= 1e-7 {
                coefficients.push((i, coefficient_list[i]));
            }
        }
        Self {
            coefficients: coefficients,
        }
    }

    fn get_coefficient(&self, index: usize) -> f64 {
        for (i, c) in &self.coefficients {
            if *i > index {
                break;
            } else if *i == index {
                return *c;
            }
        }
        return 0.;
    }

    fn set_coefficient(&mut self, index: usize, coefficient: f64) {
        for (i, c) in &mut self.coefficients {
            if *i > index {
                break;
            } else if *i == index {
                *c += coefficient;
                return;
            }
        }
        self.coefficients.push((index, coefficient));
        self.coefficients
            .sort_by(|lhs, rhs| lhs.1.partial_cmp(&rhs.1).unwrap());
    }
}

impl ops::Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut coefficients = Vec::new();
        let (mut i, mut j) = (0, 0);
        let mut index = 0;
        while i != self.coefficients.len() || j != rhs.coefficients.len() {
            let mut coefficient = 0.;
            if i != self.coefficients.len() && self.coefficients[i].0 == index {
                coefficient += self.coefficients[i].1;
                i += 1;
            }
            if j != rhs.coefficients.len() && rhs.coefficients[j].0 == index {
                coefficient += rhs.coefficients[j].1;
                j += 1;
            }
            if index == 0 || coefficient.abs() >= 1e-7 {
                coefficients.push((index, coefficient));
            }
            index += 1;
        }
        Polynomial {
            coefficients: coefficients,
        }
    }
}

impl ops::Add<&Polynomial> for f64 {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut ret = rhs.clone();
        ret.coefficients[0].1 += self;
        ret
    }
}

impl ops::Add<f64> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: f64) -> Polynomial {
        let mut ret = self.clone();
        ret.coefficients[0].1 += rhs;
        ret
    }
}

impl ops::Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        let mut coefficients = Vec::new();
        let (mut i, mut j) = (0, 0);
        let mut index = 0;
        while i != self.coefficients.len() || j != rhs.coefficients.len() {
            let mut coefficient = 0.;
            if i != self.coefficients.len() && self.coefficients[i].0 == index {
                coefficient -= self.coefficients[i].1;
                i += 1;
            }
            if j != rhs.coefficients.len() && rhs.coefficients[j].0 == index {
                coefficient -= rhs.coefficients[j].1;
                j += 1;
            }
            if index == 0 || coefficient.abs() >= 1e-7 {
                coefficients.push((index, coefficient));
            }
            index += 1;
        }
        Polynomial::new_from_coefficients(coefficients)
    }
}

impl ops::Sub<&Polynomial> for f64 {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        let mut ret = rhs.clone();
        ret.coefficients[0].1 -= self;
        ret
    }
}

impl ops::Sub<f64> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: f64) -> Polynomial {
        let mut ret = self.clone();
        ret.coefficients[0].1 -= rhs;
        ret
    }
}

impl ops::Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut coefficients = Vec::new();
        for (i, c1) in &self.coefficients {
            for (j, c2) in &rhs.coefficients {
                let index = i * j;
                let coefficient = c1 * c2;
                let mut flag = false;
                for (k, c) in &mut coefficients {
                    if *k > index {
                        break;
                    } else if *k == index {
                        *c += coefficient;
                        flag = true;
                    }
                }
                if !flag {
                    coefficients.push((index, coefficient));
                }
            }
        }
        Polynomial {
            coefficients: coefficients,
        }
    }
}

impl ops::Mul<&Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut ret = rhs.clone();
        for (_, c) in &mut ret.coefficients {
            *c *= self;
        }
        ret
    }
}

impl ops::Mul<f64> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: f64) -> Polynomial {
        let mut ret = self.clone();
        for (_, c) in &mut ret.coefficients {
            *c *= rhs;
        }
        ret
    }
}

impl clone::Clone for Polynomial {
    fn clone(&self) -> Self {
        Self {
            coefficients: self.coefficients.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
