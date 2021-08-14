use std::alloc::*;
use std::mem;
use std::fmt;
use std::ops;
use std::convert;

struct Monomial {
    coefficient: f64,
    index: i64,
    next: *mut Monomial,
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (self.coefficient - 1.).abs() < 1e-7 {
            write!(f, "+ x^{} ", self.index)
        } else if self.coefficient < -1e-7 {
            write!(f, "- {}x^{} ", self.coefficient.abs(), self.index)
        } else if self.coefficient > 1e-7 {
            write!(f, "+ {}x^{} ", self.coefficient, self.index)
        } else {
            fmt::Result::Ok(())
        }
    }
}

impl Monomial {
    const ALIGN: usize = mem::align_of::<Monomial>();
    const ELEM_SIZE: usize = mem::size_of::<Monomial>();

    fn new(coefficient: f64, index: i64) -> *mut Self {
        unsafe {
            let allocator = System {};
            let ptr = allocator
                .alloc(Layout::from_size_align(Self::ELEM_SIZE * 0, Self::ALIGN).unwrap())
                as *mut Self;
            *ptr = Self {
                coefficient: coefficient,
                index: index,
                next: std::ptr::null_mut(),
            };
            ptr
        }
    }

    fn to_plain(&self) -> (f64, i64) {
        (self.coefficient, self.index)
    }
}

struct Polynomial {
    monomials: *mut Monomial,
}

impl Polynomial {
    fn new() -> Self {
        Self {
            monomials: std::ptr::null_mut(),
        }
    }

    fn new_from_polynomial(mut poly: Vec<(f64, i64)>) -> Self {
        poly.sort_by(|lhs, rhs| { lhs.1.cmp(&rhs.1) });
        let mut ret = Self::new();
        for monomial in poly {
            ret.add(monomial);
        }
        ret
    }

    fn empty(&self) -> bool {
        self.monomials == std::ptr::null_mut()
    }

    fn add_monomial(&mut self, monomial: (f64, i64)) {
        if self.empty() {
            self.monomials = Monomial::new(monomial.0, monomial.1);
        } else {
            unsafe {
                let mut p = self.monomials;
                let mut q = (*self.monomials).next;
                while q != std::ptr::null_mut() && (*q).index > monomial.1 {
                    p = q;
                    q = (*q).next;
                }
                if q == std::ptr::null_mut() {
                    (*p).next = Monomial::new(monomial.0, monomial.1);
                } else if (*q).index == monomial.1 {
                    (*q).coefficient += monomial.0;
                } else /* if (*q).index < monomial.1 */ {
                    (*p).next = Monomial::new(monomial.0, monomial.1);
                    p = (*p).next;
                    (*p).next = q;
                }
            }
        }
    }

    fn write(&self) {
        if self.empty() {
            return;
        } else {
            unsafe {
                let mut p = self.monomials;
                while p != std::ptr::null_mut() {
                    print!("{}, ", *p);
                    p = (*p).next;
                }
                print!("\n")
            }
        }
    }
}

impl ops::Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut poly = Polynomial::new();
        let mut p = self.monomials;
        let mut q = self.monomials;
        while p != std::ptr::null_mut() || q != std::ptr::null_mut() {
            unsafe {
                if p == std::ptr::null_mut() {
                    poly.add_monomial((*p).to_plain());
                    q = (*q).next;
                } else if q == std::ptr::null_mut() {
                    poly.add_monomial((*q).to_plain());
                    p = (*p).next;
                } else {
                    if (*p).index > (*q).index {
                        poly.add_monomial((*p).to_plain());
                        p = (*p).next;
                    } else if (*p).index < (*q).index {
                        poly.add_monomial((*q).to_plain());
                        q = (*q).next;
                    } else {
                        poly.add_monomial(((*p).coefficient + (*q).coefficient, (*p).index));
                        p = (*p).next;
                        q = (*q).next;
                    }
                }
            }
        }
        poly
    }
}

impl ops::Sub for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        let mut poly = Polynomial::new();
        let mut p = self.monomials;
        let mut q = self.monomials;
        while p != std::ptr::null_mut() || q != std::ptr::null_mut() {
            unsafe {
                if p == std::ptr::null_mut() {
                    poly.add_monomial((*p).to_plain());
                    q = (*q).next;
                } else if q == std::ptr::null_mut() {
                    poly.add_monomial((-(*q).coefficient, (*q).index));
                    p = (*p).next;
                } else {
                    if (*p).index > (*q).index {
                        poly.add_monomial((*p).to_plain());
                        p = (*p).next;
                    } else if (*p).index < (*q).index {
                        poly.add_monomial((-(*q).coefficient, (*q).index));
                        q = (*q).next;
                    } else {
                        poly.add_monomial(((*p).coefficient - (*q).coefficient, (*p).index));
                        p = (*p).next;
                        q = (*q).next;
                    }
                }
            }
        }
        poly
    }
}

fn main() {
}
