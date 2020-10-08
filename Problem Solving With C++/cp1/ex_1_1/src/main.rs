use std::io;

fn main() {
    println!("Please input your first number: ");
    let mut n1_line = String::new();
    io::stdin().read_line(&mut n1_line)
        .expect("Filed to read line!");
    let n1 : f64 = match n1_line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };

    println!("Please input your second number: ");
    let mut n2_line = String::new();
    io::stdin().read_line(&mut n2_line)
        .expect("Failed to read line!");
    let n2 : f64 = match n2_line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };

    println!("Sum of {} and {} is {}", n1, n2, n1 + n2);
    println!("Product of {} and {} is {}", n1, n2, n1 * n2);
}
