#[macro_use]
extern crate scan_fmt;

mod complex;

use complex::Complex;

fn main() {
    let complex = Complex::new_from_complex(1., 1.);
    println!("{}", complex);
}
