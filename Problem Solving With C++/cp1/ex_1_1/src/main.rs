use std::io;

fn read_number() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Filed to read line!");
    let n : f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };
    n
}

fn main() {
    println!("Please input your first number: ");
    let n1 = read_number();

    println!("Please input your second number: ");
    let n2 = read_number();

    println!("Sum of {} and {} is {}", n1, n2, n1 + n2);
    println!("Product of {} and {} is {}", n1, n2, n1 * n2);
}
