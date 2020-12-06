use std::io;

fn read_f64() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    match line.trim().parse::<f64>() {
        Ok(num) => match num {
            num if num > 0. => num,
            _ => panic!("Not negative!"),
        },
        Err(_) => panic!("Failed to parse to number!"),
    }
}

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

fn calculate_hat_size(weight: f64, height: f64) -> f64 {
    weight / height * 2.9
}

fn calculate_cloth_size(weight: f64, height: f64, age: u64) -> f64 {
    height * weight / 288.
        + match age {
            age if age >= 30 => ((age - 30) / 10) as f64 / 8.,
            _ => 0.,
        }
}

fn calculate_waistline(weight: f64, age: u64) -> f64 {
    weight / 5.7
        + match age {
            age if age >= 28 => ((age - 28) / 2) as f64 / 10.,
            _ => 0.,
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
        println!("Enter weight (pounds): ");
        let weight = read_f64();

        println!("Enter height (inch): ");
        let height = read_f64();

        println!("Enter age: ");
        let age = read_u64();

        println!("Hat size after 10 years: {}", calculate_hat_size(weight, height));
        println!("Cloth size after 10 years: {}", calculate_cloth_size(weight, height, age + 10));
        println!("Waistline after 10 years: {}", calculate_waistline(weight, age + 10));

        println!("Continue(y)?: ");
        match read_confirm() {
            false => break,
            _ => continue,
        }
    }
}
