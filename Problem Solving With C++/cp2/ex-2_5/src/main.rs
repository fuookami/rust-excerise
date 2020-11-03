use std::i64;
use std::io;

fn read_amount() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let amount: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match amount {
        amount if amount >= 0 => amount,
        _ => panic!("Not negative amount!"),
    }
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    loop {
        println!("Please input population capacity: ");
        let capacity = read_amount();

        println!("Please input participant amount: ");
        let amount = read_amount();

        match capacity - amount {
            rest if rest >= 0 => println!("Legal! Rest {}", rest),
            excess => println!("Illegal! Excess {}", i64::abs(excess)),
        }

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
