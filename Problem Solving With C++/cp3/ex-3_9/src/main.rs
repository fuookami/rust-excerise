use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut count = 0;
    for _ in 0..10000 {
        let target = rng.gen_range(0, 3);
        let select = rng.gen_range(0, 3);
        if target == select {
            count += 1;
        }
    }
    println!("Bear the palm: {} / 10000", count);

    count = 0;
    for _ in 0..10000 {
        let target = rng.gen_range(0, 3);
        let select = rng.gen_range(0, 3);
        if select != target {
            count += 1;
        }
    }
    println!("Bear the palm: {} / 10000", count);
}
