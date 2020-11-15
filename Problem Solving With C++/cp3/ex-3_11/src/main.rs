use std::io;

fn read_u64() -> u64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<u64>() {
        Ok(num) => match num {
            num if num <= 999u64 => num,
            _ => panic!("Unvalid number!"),
        },
        Err(_) => panic!("Failed to parse to number!"),
    }
}

fn has_disabled_number(number: u64) -> bool {
    static DISABLED_NUMBERS: [u64; 3] = [1, 4, 7];
    for disabled_number in DISABLED_NUMBERS.iter() {
        if number / 100 == *disabled_number {
            return true;
        }
        if (number % 100) / 10 == *disabled_number {
            return true;
        }
        if number % 10 == *disabled_number {
            return true;
        }
    }
    return false;
}

fn find_min_enabled_number(number: u64) -> Option<u64> {
    if !has_disabled_number(number) {
        return Option::Some(number);
    }

    for i in (0..number).rev() {
        if !has_disabled_number(i) {
            return Option::Some(i);
        }
    }
    return Option::None;
}

fn find_max_enabled_number(number: u64) -> Option<u64> {
    if !has_disabled_number(number) {
        return Option::Some(number);
    }

    for i in (number + 1)..=999 {
        if !has_disabled_number(i) {
            return Option::Some(i);
        }
    }
    return Option::None;
}

fn main() {
    println!("Enter temperature: ");
    let number = read_u64();

    match find_min_enabled_number(number) {
        Option::Some(number) => println!("Min enabled number: {}", number),
        Option::None => println!("No min enabled number."),
    }

    match find_max_enabled_number(number) {
        Option::Some(number) => println!("Max enabled number: {}", number),
        Option::None => println!("No max enabled number."),
    }
}
