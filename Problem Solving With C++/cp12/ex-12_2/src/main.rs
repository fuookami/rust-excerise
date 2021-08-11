mod rational;

use rational::Rational;

fn main() {
    let r1 = Rational::new_from_rational(2, 4);
    let r2 = Rational::new_from_rational(1, 2);
    assert!(r1 == r2);
}
