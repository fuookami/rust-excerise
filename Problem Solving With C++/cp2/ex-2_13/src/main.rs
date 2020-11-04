use std::io;

enum Sex {
    Male,
    Female,
}

fn calculate_bmr_female(weight: f64, height: f64, age: i64) -> f64 {
    655. + 4.3 * weight + 4.7 * height - 4.7 * (age as f64)
}

fn calculate_bmr_male(weight: f64, height: f64, age: i64) -> f64 {
    66. + 6.3 * weight + 12.9 * height - 6.8 * (age as f64)
}

fn read_weight() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let weight: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match weight {
        weight if weight >= 0. => weight,
        _ => panic!("Not negative weight!"),
    }
}

fn read_height() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let height: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match height {
        height if height >= 0. => height,
        _ => panic!("Not negative height!"),
    }
}

fn read_age() -> i64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let age: i64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match age {
        age if age >= 0 => age,
        _ => panic!("Not negative age!"),
    }
}

fn read_sex() -> Sex {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line {
        _ if line.trim() == "M" => Sex::Male,
        _ if line.trim() == "F" => Sex::Female,
        _ => panic!("Invalid Sex!"),
    }
}

fn main() {
    println!("Enter your weight: ");
    let weight = read_weight();

    println!("Enter your height: ");
    let height = read_height();

    println!("Enter your age: ");
    let age = read_age();

    println!("Enter your sex(M, F): ");
    let sex = read_sex();

    let bmr = match sex {
        Sex::Male => calculate_bmr_male(weight, height, age),
        Sex::Female => calculate_bmr_female(weight, height, age),
    };
    println!("Chocolate amount: {}", (bmr / 230.).ceil() as i64);
}
