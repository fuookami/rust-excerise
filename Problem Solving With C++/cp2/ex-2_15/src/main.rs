use std::io;

fn calculate_linear_displacement(alpha: f64, length: f64, temperature_variation: f64) -> f64 {
    alpha * length * temperature_variation
}

fn print_linear_displacement(linear_displacement: f64) {
    match linear_displacement {
        num if linear_displacement >= 0. => {
            println!("The material will expand by {:.2} meters", num)
        }
        num => print!("The material will constract by {:.2} meters", num),
    }
}

fn read_alpha() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let alpha: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match alpha {
        alpha if alpha >= 0. => alpha,
        _ => panic!("Not negative alpha!"),
    }
}

fn read_length() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let length: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    match length {
        length if length >= 0. => length,
        _ => panic!("Not negative length!"),
    }
}

fn read_temperature_variation() -> f64 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let temperature_variation: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse to number!"),
    };
    temperature_variation
}

fn main() {
    println!("Enter coefficient of linear thermal expansion: ");
    let alpha = read_alpha();

    println!("Enter length: ");
    let length = read_length();

    println!("Enter temperature variation: ");
    let temperature_variation = read_temperature_variation();

    print_linear_displacement(
        calculate_linear_displacement(alpha, length, temperature_variation)
    );
}
