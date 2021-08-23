fn factorial(val: u64) -> u64 {
    if val == 0 {
        1
    } else {
        val * factorial(val - 1)
    }
}

fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    println!("Hello, world!");
}
