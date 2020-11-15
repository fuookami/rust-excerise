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

fn calculate_weight(mut weight: u64, days: u64) -> u64 {
    let mut last_weight = weight;
    for i in 1..days {
        if i % 5 == 0 {
            let last_last_weight = last_weight;
            last_weight = weight;
            weight = last_last_weight + last_weight;
        }
    }
    weight
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
        println!("Enter weight: ");
        let weight = read_u64();

        println!("Enter days: ");
        let days = read_u64();

        let after_weight = calculate_weight(weight, days);
        println!("Weight after {} days: {}", days, after_weight);

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
