use rand::distributions::Uniform;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let dis = Uniform::new(0.0f64, 1.0);
    let mut v = vec![];
    for _ in 0..10 {
        v.push(dis.sample(&mut rng));
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for val in v {
        print!("{}, ", val);
    }
}
