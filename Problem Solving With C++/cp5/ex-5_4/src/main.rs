use std::io;

struct Change {
    quarter: u32,
    dime: u32,
    penny: u32,
}

impl Change {
    fn compute_coins(coin_value: u32, amount_left: u32) -> (u32, u32) {
        (amount_left / coin_value, amount_left % coin_value)
    }

    fn new(amount: u32) -> Change {
        let (quarter, left) = Self::compute_coins(25, amount);
        let (dime, left) = Self::compute_coins(10, left);
        let (penny, _) = Self::compute_coins(1, left);
        Change {
            quarter: quarter,
            dime: dime,
            penny: penny,
        }
    }
}

fn read_u32() -> u32 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn main() {
    println!("Enter amount: ");
    let amount = read_u32();
    let change = Change::new(amount);
    println!(
        "{} cents can be given as {} quarter(s) {} dime(s) and {} penny(pennies)",
        amount, change.quarter, change.dime, change.penny
    );
}
