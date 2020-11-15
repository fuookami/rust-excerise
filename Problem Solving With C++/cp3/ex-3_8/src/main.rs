use std::io;

fn read_u64() -> u64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn calculate_pi(n: u64) -> f64 {
    let mut sum = 0.;
    for i in 0..n {
        if i % 2 == 0 {
            sum += 1. / (2 * i + 1) as f64;
        } else {
            sum -= 1. / (2 * i + 1) as f64;
        }
    }
    sum * 4.
}

fn main() {
    loop {
        println!("Enter n: ");
        let n = read_u64();

        println!("Pi: {:.15}", calculate_pi(n));

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
