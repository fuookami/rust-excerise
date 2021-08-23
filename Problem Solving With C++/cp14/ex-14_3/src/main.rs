fn factorial(val: u64) -> u64 {
    if val == 0 {
        1
    } else {
        let mut ret = 1;
        for i in 0..=val {
            ret *= i;
        }
        ret
    }
}

fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    println!("Hello, world!");
}
