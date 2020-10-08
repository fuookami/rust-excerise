use std::io;

fn read_amount() -> i64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line!");
    let amount : i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!")
    };
    return amount
}

fn main() {
    println!("Please input amount of 25 cent coin: ");
    let amount25 = read_amount();

    println!("Please input amount of 10 cent coin: ");
    let amount10 = read_amount();

    println!("Please input amount of 5 cent coin: ");
    let amount5 = read_amount();

    println!("Total {} cent.", amount25 * 25 + amount10 * 10 + amount5 * 5);
}
